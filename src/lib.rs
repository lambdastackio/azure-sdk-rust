// Copyright 2017 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// NOTE: This attribute only needs to be set once.
#![doc(html_logo_url = "https://lambdastackio.github.io/static/images/lambdastack-200x200.png",
       html_favicon_url = "https://lambdastackio.github.io/static/images/favicon.ico",
       html_root_url = "https://lambdastackio.github.io/azure-sdk-rust/azure_sdk_rust/azure/index.html")]

// #![feature(plugin)]
// #![plugin(clippy)]

#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize as serialize;
extern crate xml;
extern crate uuid;
extern crate time;

#[macro_use]
extern crate mime;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
pub mod azure;
