// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{on_disk::OnDiskStorage, tests::suite};
use libra_temppath::TempPath;

#[test]
fn on_disk() {
    let temp_path = TempPath::new();
    let storage = Box::new(OnDiskStorage::new(temp_path.path().to_path_buf()));
    suite::run_test_suite(storage);
}
