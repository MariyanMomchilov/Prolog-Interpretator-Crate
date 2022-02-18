#[cfg(test)]
mod tests;

use super::parser::{Constant, Fact, Rule, Variable};
use std::{collections::HashMap, fmt::Debug};

struct IdFactory {
    id_counter: u32,
}

impl IdFactory {
    pub fn id(&mut self) -> u32 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }
}

pub trait Clause: Debug + Unify {
    fn copy(&self) -> Box<dyn Clause>;

    fn get_variable_names(&self, _: &mut Vec<String>) {}
}

pub trait Unify {
    fn unify(
        &self,
        rhs: &dyn Clause,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
    ) -> Option<Box<dyn Clause>>;

    fn get_name(&self) -> Option<String> {
        None
    }

    fn get_args(&self) -> Option<Vec<Box<dyn Clause>>> {
        None
    }

    fn apply_mapping(
        &self,
        _variable_mapping: &HashMap<String, Box<dyn Clause>>,
    ) -> Box<dyn Clause>;

    fn subgoals(&self) -> Option<Vec<Box<dyn Clause>>> {
        None
    }
}

impl Clause for Variable {
    fn copy(&self) -> Box<dyn Clause> {
        Box::new(Variable(self.0.clone()))
    }

    fn get_variable_names(&self, names: &mut Vec<String>) {
        if !names.contains(&self.0) {
            names.push(self.0.clone());
        }
    }
}

impl Unify for Variable {
    fn unify(
        &self,
        rhs: &dyn Clause,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
    ) -> Option<Box<dyn Clause>> {
        let name = rhs.get_name();
        // rhs is Variable
        if let None = name {
            let mut varname_singleton: Vec<String> = Vec::new();
            rhs.get_variable_names(&mut varname_singleton);
            if variable_mapping.contains_key(&self.0) {
                if variable_mapping.contains_key(&varname_singleton[0]) {
                    return None;
                }
                variable_mapping.insert(varname_singleton[0].clone(), self.copy());
            } else {
                variable_mapping.insert(self.0.clone(), rhs.copy());
            }
            return Some(self.copy());
        }

        // to do: fix rhs.get_args(), will always return None
        if let Some(_) = rhs.get_args() {
            variable_mapping.insert(self.0.clone(), rhs.copy());
            return Some(rhs.copy());
        }

        // rhs is constant
        variable_mapping.insert(self.0.clone(), rhs.copy());
        return Some(rhs.copy());
    }

    fn apply_mapping(
        &self,
        variable_mapping: &HashMap<String, Box<dyn Clause>>,
    ) -> Box<dyn Clause> {
        if let Some(c) = variable_mapping.get(&self.0) {
            return c.copy();
        }
        self.copy()
    }
}

impl Clause for Constant {
    fn copy(&self) -> Box<dyn Clause> {
        Box::new(Constant(self.0.clone()))
    }
}

impl Unify for Constant {
    fn get_name(&self) -> Option<String> {
        Some(self.0.clone())
    }

    fn unify(
        &self,
        rhs: &dyn Clause,
        _variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
    ) -> Option<Box<dyn Clause>> {
        let args = rhs.get_args();
        if let None = args {
            let rname = rhs.get_name();
            if self.get_name() != rname && rname != None {
                return None;
            }
            return Some(Box::new(Constant(self.0.clone())));
        }
        None
    }

    fn apply_mapping(
        &self,
        _variable_mapping: &HashMap<String, Box<dyn Clause>>,
    ) -> Box<dyn Clause> {
        self.copy()
    }
}

impl Clause for Fact {
    fn copy(&self) -> Box<dyn Clause> {
        let mut args_copy = Vec::new();
        for arg in self.args.iter() {
            args_copy.push(arg.copy());
        }
        Box::new(Fact {
            name: self.name.clone(),
            args: args_copy,
        })
    }

    fn get_variable_names(&self, names: &mut Vec<String>) {
        for arg in self.args.iter() {
            arg.get_variable_names(names);
        }
    }
}

impl Unify for Fact {
    fn unify(
        &self,
        rhs: &dyn Clause,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
    ) -> Option<Box<dyn Clause>> {
        // rhs is functor
        if let Some(s) = rhs.get_name() {
            if s == self.name {
                if let Some(rargs) = rhs.get_args() {
                    if self.args.len() != rargs.len() {
                        return None;
                    }
                    let mut i = 0;
                    let mut new_args: Vec<Box<dyn Clause>> = Vec::new();
                    for arg in self.args.iter() {
                        let unified_arg = arg.unify(rargs[i].as_ref(), variable_mapping);
                        match unified_arg {
                            None => return None,
                            Some(u) => new_args.push(u),
                        };
                        i += 1;
                    }
                    return Some(Box::new(Fact {
                        name: self.name.clone(),
                        args: new_args,
                    }));
                }
                return None;
            }
        }
        rhs.unify(self, variable_mapping)
    }

    fn get_name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn get_args(&self) -> Option<Vec<Box<dyn Clause>>> {
        let mut args_copy = Vec::new();
        for arg in self.args.iter() {
            args_copy.push(arg.copy());
        }
        Some(args_copy)
    }

    fn apply_mapping(
        &self,
        variable_mapping: &HashMap<String, Box<dyn Clause>>,
    ) -> Box<dyn Clause> {
        let mut new_args = Vec::new();
        for arg in self.args.iter() {
            new_args.push(arg.apply_mapping(&variable_mapping));
        }
        Box::new(Fact {
            name: self.name.clone(),
            args: new_args,
        })
    }
}

impl Clause for Rule {
    fn copy(&self) -> Box<dyn Clause> {
        let head_copy = self.head.copy();
        let mut body_copy = Vec::new();
        for c in self.body.iter() {
            body_copy.push(c.copy());
        }
        Box::new(Rule {
            head: head_copy,
            body: body_copy,
        })
    }

    fn get_variable_names(&self, names: &mut Vec<String>) {
        self.head.get_variable_names(names);
        for clauses in self.body.iter() {
            clauses.get_variable_names(names);
        }
    }
}

impl Unify for Rule {
    fn unify(
        &self,
        rhs: &dyn Clause,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
    ) -> Option<Box<dyn Clause>> {
        let unified_head = self.head.unify(rhs, variable_mapping)?;

        let mut unified_body = Vec::new();
        for i in 0..self.body.len() {
            let replaced = self.body[i].apply_mapping(variable_mapping);
            unified_body.push(replaced);
        }
        Some(Box::new(Rule {
            head: unified_head,
            body: unified_body,
        }))
    }

    fn get_name(&self) -> Option<String> {
        self.head.get_name()
    }

    fn get_args(&self) -> Option<Vec<Box<dyn Clause>>> {
        self.head.get_args()
    }

    fn apply_mapping(
        &self,
        variable_mapping: &HashMap<String, Box<dyn Clause>>,
    ) -> Box<dyn Clause> {
        let mapped_head = self.head.apply_mapping(&variable_mapping);
        let mut mapped_body = Vec::new();
        for c in self.body.iter() {
            mapped_body.push(c.apply_mapping(&variable_mapping));
        }
        Box::new(Rule {
            head: mapped_head,
            body: mapped_body,
        })
    }

    fn subgoals(&self) -> Option<Vec<Box<dyn Clause>>> {
        let mut vec = Vec::new();
        for c in self.body.iter() {
            vec.push(c.copy());
        }
        Some(vec)
    }
}

struct Ctx {
    at_goal: usize,
    goal: Box<dyn Clause>,
    subgoals: Vec<u32>,
    id: u32,
    parent_id: u32,
}

impl Ctx {
    pub fn from_goal(goal: Box<dyn Clause>, id: u32, parent_id: u32) -> Ctx {
        Ctx {
            at_goal: 0,
            goal,
            subgoals: Vec::new(),
            id,
            parent_id,
        }
    }

    pub fn advance(
        &mut self,
        clauses: &Vec<Box<dyn Clause>>,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
        ctx_mapping: &mut HashMap<u32, Ctx>,
        id_factory: &mut IdFactory
    ) {
        if self.at_goal == self.subgoals.len() {
            if ctx_mapping.contains_key(&self.parent_id) {
                ctx_mapping.get_mut(&self.parent_id).unwrap().advance(clauses, variable_mapping, ctx_mapping, id_factory);
            }
        }
        else {

        }
    }

    pub fn evaluate(
        &mut self,
        clauses: &Vec<Box<dyn Clause>>,
        variable_mapping: &mut HashMap<String, Box<dyn Clause>>,
        ctx_mapping: &mut HashMap<u32, Ctx>,
        id_factory: &mut IdFactory,
    ) {
        for clause in clauses.iter() {
            if let Some(matched_clause) = clause.unify(&*self.goal, variable_mapping) {
                Runner::replace_variables(&matched_clause, variable_mapping);
                let new_goal = matched_clause.apply_mapping(variable_mapping);
                let subgoals = new_goal.subgoals();
                if let Some(goals) = subgoals {
                    for goal in goals.iter() {
                        let id = id_factory.id();
                        ctx_mapping.insert(id, Ctx {
                            at_goal: 0,
                            goal: goal.copy(),
                            subgoals: Vec::new(),
                            id: id_factory.id(),
                            parent_id: self.id,
                        });
                        self.subgoals.push(id);
                    }
                }
                self.advance(clauses, variable_mapping, ctx_mapping, id_factory);
            }
        }
    }
}

pub struct Runner {
    goal: Box<dyn Clause>,
    clauses: Vec<Box<dyn Clause>>,
    variable_mapping: HashMap<String, Box<dyn Clause>>,
    ctx_mapping: HashMap<u32, Ctx>,
    id_factory: IdFactory,
}

impl Runner {
    pub fn from_input(goal: Box<dyn Clause>, clauses: Vec<Box<dyn Clause>>) -> Runner {
        Runner {
            goal,
            clauses,
            variable_mapping: HashMap::new(),
            ctx_mapping: HashMap::new(),
            id_factory: IdFactory { id_counter: 1 },
        }
    }

    fn replace_variables(clause: &Box<dyn Clause>, mapping: &mut HashMap<String, Box<dyn Clause>>) {
        let mut variables = Vec::new();
        clause.get_variable_names(&mut variables);

        for variable in variables.iter() {
            let mut delegated_var = variable.clone();
            delegated_var.push_str("_");
            mapping.insert(variable.clone(), Box::new(Variable(delegated_var)));
        }
    }

    fn run(&mut self) {
        let mut ctx = Ctx::from_goal(self.goal.copy(), self.id_factory.id(), 0);
        ctx.evaluate(&self.clauses, &mut self.variable_mapping, &mut self.ctx_mapping, &mut self.id_factory);
    }
}
