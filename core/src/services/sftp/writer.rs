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

use async_trait::async_trait;
use openssh_sftp_client::file::File;

use crate::raw::oio;
use crate::*;

pub struct SftpWriter {
    file: File,
}

impl SftpWriter {
    pub fn new(file: File) -> Self {
        SftpWriter { file }
    }
}

#[async_trait]
impl oio::Write for SftpWriter {
    async fn write(&mut self, bs: &dyn oio::WriteBuf) -> Result<usize> {
        let size = self.file.write(bs.chunk()).await?;

        Ok(size)
    }

    async fn abort(&mut self) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "SFTP does not support aborting writes",
        ))
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
