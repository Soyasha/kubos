/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;
use crate::mai400::*;

#[test]
fn reset_good() {
    let mut mock = MockStream::default();

    // Packet for RequestReset
    mock.write.set_input(vec![
        0x90, 0xEB, 0x5A, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xD5, 0x1,
    ]);

    // Packet for ConfirmReset
    mock.write.set_input(vec![
        0x90, 0xEB, 0xF1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x6C, 0x2,
    ]);

    let mai = mock_new!(mock);

    assert_eq!(mai.reset().unwrap(), ());
}

#[test]
fn reset_bad() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    assert_eq!(
        mai.reset().unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn set_mode_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0x90, 0xEB, 0x0, 0x1, 0x2, 0x0, 0x3, 0x0, 0x4, 0x0, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x8A, 0x1,
    ]);

    let mai = mock_new!(mock);

    assert_eq!(mai.set_mode(0x01, [0x02, 0x03, 0x04, 0x05]).unwrap(), ());
}

#[test]
fn set_mode_bad() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    assert_eq!(
        mai.set_mode(0x01, [0x02, 0x03, 0x04, 0x05]).unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn set_mode_sun_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0x90, 0xEB, 0x0, 0x8, 0x1, 0x0, 0xCD, 0xCC, 0xC, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x69, 0x3,
    ]);

    let mai = mock_new!(mock);

    assert_eq!(mai.set_mode_sun(0x08, 1, 2.2).unwrap(), ());
}

#[test]
fn set_mode_sun_bad() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    assert_eq!(
        mai.set_mode_sun(0x08, 1, 2.2).unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn set_gps_time_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0x90, 0xEB, 0x44, 0x92, 0x3C, 0x74, 0x47, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x48, 0x3,
    ]);

    let mai = mock_new!(mock);

    assert_eq!(mai.set_gps_time(1198800018).unwrap(), ());
}

#[test]
fn set_gps_time_bad() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    assert_eq!(
        mai.set_gps_time(15).unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn set_rv_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0x90, 0xEB, 0x41, 0xCD, 0xCC, 0x8C, 0x3F, 0xCD, 0xCC, 0xC, 0x40, 0x33, 0x33, 0x53, 0x40,
        0xCD, 0xCC, 0x8C, 0x40, 0x0, 0x0, 0xB0, 0x40, 0x33, 0x33, 0xD3, 0x40, 0x92, 0x3C, 0x74,
        0x47, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x55, 0xD,
    ]);

    let mai = mock_new!(mock);

    assert_eq!(
        mai.set_rv([1.1, 2.2, 3.3], [4.4, 5.5, 6.6], 1198800018)
            .unwrap(),
        ()
    );
}

#[test]
fn set_rv_bad() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    assert_eq!(
        mai.set_rv([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0).unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn passthrough_good() {
    let mut mock = MockStream::default();

    let msg: [u8; 40] = [0x00; 40];

    mock.write.set_input(msg.to_vec());

    let mai = mock_new!(mock);

    assert_eq!(mai.passthrough(&msg).unwrap(), ());
}

#[test]
fn passthrough_bad() {
    let mock = MockStream::default();

    let msg: [u8; 40] = [0x00; 40];

    let mai = mock_new!(mock);

    assert_eq!(
        mai.passthrough(&msg).unwrap_err(),
        MAIError::UartError {
            cause: UartError::GenericError,
        }
    );
}
