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
    lexis::{Length, Site, SiteSpan, SourceCode, ToSite, ToSpan, Token},
    std::*,
};

/// A Token metadata borrow object.
///
/// This object borrows reference into the Token instance, and the metadata of the source code
/// substring this token belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chunk<'source, T: Token> {
    /// Token instance reference.
    ///
    /// This instance is supposed to describe lexical kind of the "token", and possible additional
    /// generic semantic metadata inside this instance.
    pub token: T,

    /// Token's substring absolute UTF-8 character offset inside the source code text.
    pub site: Site,

    /// Token's substring UTF-8 characters count.
    pub length: Length,

    /// Token's original substring reference inside the source code text.
    pub string: &'source str,
}

impl<'source, T: Token> Chunk<'source, T> {
    #[inline(always)]
    pub fn start(&self) -> Site {
        self.site
    }

    #[inline(always)]
    pub fn end(&self) -> Site {
        self.site + self.length
    }
}

unsafe impl<'source, T: Token> ToSpan for Chunk<'source, T> {
    #[inline(always)]
    fn to_site_span(&self, code: &impl SourceCode) -> Option<SiteSpan> {
        let start = unsafe { self.start().to_site(code).unwrap_unchecked() };
        let end = unsafe { self.end().to_site(code).unwrap_unchecked() };

        Some(start..end)
    }

    #[inline(always)]
    fn is_valid_span(&self, _code: &impl SourceCode) -> bool {
        true
    }
}
