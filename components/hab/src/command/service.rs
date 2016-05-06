// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {
    pub mod generate {
        use std::path::Path;

        use hcore::crypto::BoxKeyPair;
        use hcore::service::ServiceGroup;

        use error::Result;

        pub fn start(org: &str, service_group: &ServiceGroup, cache: &Path) -> Result<()> {
            let pair = try!(BoxKeyPair::generate_pair_for_service(org,
                                                                  &service_group.to_string(),
                                                                  cache));
            println!("Successfully generated service key {}",
                     pair.name_with_rev());
            Ok(())
        }
    }
}
