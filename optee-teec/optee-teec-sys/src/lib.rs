// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(non_camel_case_types, non_snake_case)]

pub use tee_client_api::*;

mod tee_client_api;

// Pull in `imp` field definitions, which differ in size and type depending on
// the implementation we're building for. We need to know about them because
// we're the ones allocating memory for all these structures.
#[cfg(all(feature = "backend-optee", feature = "backend-isee"))]
compile_error!("at most one backend can be enabled; did you forget to disable default features?");
#[cfg_attr(feature = "backend-optee", path = "imps/optee.rs")]
#[cfg_attr(feature = "backend-isee", path = "imps/isee.rs")]
mod imp;
