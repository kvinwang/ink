// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Crate providing allocator support for all Wasm compilations of ink! smart contracts.
//!
//! The default allocator is a bump allocator whose goal is to have a small size footprint. If you
//! are not concerned about the size of your final Wasm binaries you may opt into using the more
//! full-featured `wee_alloc` allocator by activating the `wee-alloc` crate feature.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler, core_intrinsics))]

// We use `wee_alloc` as the global allocator since it is optimized for binary file size
// so that contracts compiled with it as allocator do not grow too much in size.
#[cfg(not(feature = "std"))]
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[cfg(not(feature = "std"))]
mod handlers;
