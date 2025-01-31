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

//! A builder for [`Driver`]
//!
//!

#[cfg(feature = "env")]
use std::env;

use adbc_core::{
    error::{Error, Result},
    options::AdbcVersion,
};

use crate::Driver;

/// A builder for [`Driver`].
///
/// The builder can be used to initialize a [`Driver`] with
/// [`Builder::try_load`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Builder {
    /// The [`AdbcVersion`] version of the driver.
    pub adbc_version: Option<AdbcVersion>,
}

#[cfg(feature = "env")]
impl Builder {
    /// See [`Self::adbc_version`].
    pub const ADBC_VERSION_ENV: &str = "ADBC_SNOWFLAKE_ADBC_VERSION";

    /// Construct a builder, setting values based on values of the
    /// configuration environment variables.
    pub fn from_env() -> Self {
        #[cfg(feature = "dotenv")]
        let _ = dotenvy::dotenv();

        let adbc_version = env::var(Self::ADBC_VERSION_ENV)
            .ok()
            .as_deref()
            .and_then(|value| value.parse().ok());
        Self { adbc_version }
    }
}

impl Builder {
    /// Use the provided [`AdbcVersion`] when loading the driver.
    pub fn with_adbc_version(mut self, version: AdbcVersion) -> Self {
        self.adbc_version = Some(version);
        self
    }

    /// Try to load the [`Driver`] using the values provided to this builder.
    pub fn try_load(self) -> Result<Driver> {
        Driver::try_new(self.adbc_version.unwrap_or_default())
    }
}

impl TryFrom<Builder> for Driver {
    type Error = Error;

    fn try_from(value: Builder) -> Result<Self> {
        value.try_load()
    }
}
