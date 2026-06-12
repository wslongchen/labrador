/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */

use std::collections::HashMap;

use sxd_document::dom::Document;
use sxd_document::parser;
use sxd_document::Package;
use sxd_xpath::{EvaluationContext, Factory, Functions, Namespaces, Value, Variables};


pub fn parse<T: AsRef<str>>(xml: T) -> Package {
    parser::parse(xml.as_ref()).unwrap_or(Package::new())
}

pub fn evaluate<'d, T: AsRef<str>>(package: &'d Document<'d>, xpath: T) -> Value<'d> {
    let evaluator = XPathEvaluator::new();
    evaluator.evaluate(package, xpath.as_ref())
}

struct XPathEvaluator<'d> {
    functions: Functions,
    variables: Variables<'d>,
    namespaces: Namespaces,
    factory: Factory,
}

#[allow(unused)]
impl<'d> XPathEvaluator<'d> {
    fn new() -> XPathEvaluator<'d> {
        let mut fns = HashMap::new();
        sxd_xpath::function::register_core_functions(&mut fns);
        XPathEvaluator {
            functions: fns,
            variables: HashMap::new(),
            namespaces: HashMap::new(),
            factory: Factory::new(),
        }
    }

    fn evaluate(&self, doc: &'d Document<'d>, xpath: &str) -> Value<'d> {
        let root = doc.root();
        let context = EvaluationContext::new(
            root,
            &self.functions,
            &self.variables,
            &self.namespaces,
        );
        let v = self.factory.build(xpath).unwrap_or(None).map(|xpath| xpath.evaluate(&context).ok().unwrap_or(Value::String("".to_string())));
        v.unwrap_or(Value::String("".to_string()))
    }
}