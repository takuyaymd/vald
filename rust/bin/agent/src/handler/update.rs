//
// Copyright (C) 2019-2024 vdaas.org vald team <vald@vdaas.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use proto::{
    payload::v1::{object, update},
    vald::v1::update_server,
};

#[tonic::async_trait]
impl update_server::Update for super::Agent {
    async fn update(
        &self,
        _request: tonic::Request<update::Request>,
    ) -> std::result::Result<tonic::Response<object::Location>, tonic::Status> {
        todo!()
    }

    #[doc = " Server streaming response type for the StreamUpdate method."]
    type StreamUpdateStream = crate::stream_type!(object::StreamLocation);

    #[doc = " A method to update multiple indexed vectors by bidirectional streaming.\n"]
    async fn stream_update(
        &self,
        _request: tonic::Request<tonic::Streaming<update::Request>>,
    ) -> std::result::Result<tonic::Response<Self::StreamUpdateStream>, tonic::Status> {
        todo!()
    }

    #[doc = " A method to update multiple indexed vectors in a single request.\n"]
    async fn multi_update(
        &self,
        _request: tonic::Request<update::MultiRequest>,
    ) -> std::result::Result<tonic::Response<object::Locations>, tonic::Status> {
        todo!()
    }

    #[doc = " A method to update timestamp indexed vectors in a single request.\n"]
    async fn update_timestamp(
        &self,
        _request: tonic::Request<update::TimestampRequest>,
    ) -> std::result::Result<tonic::Response<object::Location>, tonic::Status> {
        todo!()
    }
}
