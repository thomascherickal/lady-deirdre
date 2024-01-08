////////////////////////////////////////////////////////////////////////////////
// This file is a part of the "Lady Deirdre" Work,                            //
// a compiler front-end foundation technology.                                //
//                                                                            //
// This Work is a proprietary software with source available code.            //
//                                                                            //
// To copy, use, distribute, and contribute into this Work you must agree to  //
// the terms of the End User License Agreement:                               //
//                                                                            //
// https://github.com/Eliah-Lakhin/lady-deirdre/blob/master/EULA.md.          //
//                                                                            //
// The Agreement let you use this Work in commercial and non-commercial       //
// purposes. Commercial use of the Work is free of charge to start,           //
// but the Agreement obligates you to pay me royalties                        //
// under certain conditions.                                                  //
//                                                                            //
// If you want to contribute into the source code of this Work,               //
// the Agreement obligates you to assign me all exclusive rights to           //
// the Derivative Work or contribution made by you                            //
// (this includes GitHub forks and pull requests to my repository).           //
//                                                                            //
// The Agreement does not limit rights of the third party software developers //
// as long as the third party software uses public API of this Work only,     //
// and the third party software does not incorporate or distribute            //
// this Work directly.                                                        //
//                                                                            //
// AS FAR AS THE LAW ALLOWS, THIS SOFTWARE COMES AS IS, WITHOUT ANY WARRANTY  //
// OR CONDITION, AND I WILL NOT BE LIABLE TO ANYONE FOR ANY DAMAGES           //
// RELATED TO THIS SOFTWARE, UNDER ANY KIND OF LEGAL CLAIM.                   //
//                                                                            //
// If you do not or cannot agree to the terms of this Agreement,              //
// do not use this Work.                                                      //
//                                                                            //
// Copyright (c) 2022 Ilya Lakhin (Илья Александрович Лахин).                 //
// All rights reserved.                                                       //
////////////////////////////////////////////////////////////////////////////////

use crate::{
    std::*,
    syntax::{Node, NodeRule},
};

pub trait Observer {
    type Node: Node;

    fn enter_rule(&mut self, rule: NodeRule);

    fn leave_rule(&mut self, rule: NodeRule, node: &Self::Node);

    fn parse_error(&mut self);
}

pub struct DebugObserver<N: Node> {
    depth: usize,
    _phantom: PhantomData<N>,
}

impl<N: Node> Default for DebugObserver<N> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            depth: 0,
            _phantom: PhantomData,
        }
    }
}

impl<N: Node> Observer for DebugObserver<N> {
    type Node = N;

    fn enter_rule(&mut self, rule: NodeRule) {
        let indent = self.indent();
        let name = N::rule_name(rule).unwrap_or("?");

        println!("{indent} {name} {{",);

        self.depth += 1;
    }

    fn leave_rule(&mut self, rule: NodeRule, _node: &Self::Node) {
        self.depth = self.depth.checked_sub(1).unwrap_or_default();

        let indent = self.indent();
        let name = N::rule_name(rule).unwrap_or("?");

        println!("{indent} }} {name}",);
    }

    fn parse_error(&mut self) {
        let indent = self.indent();
        println!("{indent} --- error ---",);
    }
}

impl<N: Node> DebugObserver<N> {
    #[inline(always)]
    fn indent(&self) -> String {
        "    ".repeat(self.depth)
    }
}

#[repr(transparent)]
pub(super) struct VoidObserver<N: Node>(PhantomData<N>);

impl<N: Node> Default for VoidObserver<N> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<N: Node> Observer for VoidObserver<N> {
    type Node = N;

    #[inline(always)]
    fn enter_rule(&mut self, _rule: NodeRule) {}

    #[inline(always)]
    fn leave_rule(&mut self, _rule: NodeRule, _node: &Self::Node) {}

    #[inline(always)]
    fn parse_error(&mut self) {}
}
