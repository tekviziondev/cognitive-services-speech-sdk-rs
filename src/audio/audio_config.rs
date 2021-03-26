use crate::audio::{AudioInputStream, AudioOutputStream};
use crate::common::{PropertyCollection, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_a_microphone,
    audio_config_create_audio_input_from_default_microphone,
    audio_config_create_audio_input_from_stream,
    audio_config_create_audio_input_from_wav_file_name,
    audio_config_create_audio_output_from_default_speaker,
    audio_config_create_audio_output_from_stream,
    audio_config_create_audio_output_from_wav_file_name, audio_config_get_property_bag,
    audio_config_release, SmartHandle, SPXAUDIOCONFIGHANDLE, SPXPROPERTYBAGHANDLE,
};
use log::*;
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AudioConfig {
    pub handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    pub properties: PropertyCollection,
}

impl AudioConfig {
    fn from_handle(handle: SPXAUDIOCONFIGHANDLE) -> Result<AudioConfig> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = audio_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "AudioConfig::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle);

            let result = AudioConfig {
                handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
                properties: property_bag,
            };
            Ok(result)
        }
    }

    pub fn from_stream_input(stream: &dyn AudioInputStream) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = audio_config_create_audio_input_from_stream(&mut handle, stream.get_handle());
            convert_err(ret, "AudioConfig::from_stream_input error")?;
            info!("from_stream_input ok");
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_wav_file_input(file_name: &str) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_file_name = CString::new(file_name)?;
            convert_err(
                audio_config_create_audio_input_from_wav_file_name(
                    &mut handle,
                    c_file_name.as_ptr(),
                ),
                "AudioConfig::from_wav_file_input error",
            )?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_default_microphone_input() -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            convert_err(
                audio_config_create_audio_input_from_default_microphone(&mut handle),
                "AudioConfig::from_default_microphone_input",
            )?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_microphone_input(device_name: &str) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_device_name = CString::new(device_name)?;
            convert_err(
                audio_config_create_audio_input_from_a_microphone(
                    &mut handle,
                    c_device_name.as_ptr(),
                ),
                "AudioConfig::from_microphone_input",
            )?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_default_speaker_output() -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            convert_err(
                audio_config_create_audio_output_from_default_speaker(&mut handle),
                "AudioConfig::from_default_speaker_output",
            )?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_wav_file_output(file_name: &str) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_file_name = CString::new(file_name)?;
            convert_err(
                audio_config_create_audio_output_from_wav_file_name(
                    &mut handle,
                    c_file_name.as_ptr(),
                ),
                "AudioConfig::from_wav_file_output",
            )?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn from_stream_output(stream: &dyn AudioOutputStream) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let ret =
                audio_config_create_audio_output_from_stream(&mut handle, stream.get_handle());
            convert_err(ret, "AudioConfig::from_stream_output error")?;
            AudioConfig::from_handle(handle)
        }
    }

    pub fn set_property(&mut self, id: PropertyId, value: &str) -> Result<()> {
        self.properties.set_property(id, value)
    }

    pub fn get_property(&self, id: PropertyId) -> Result<String> {
        self.properties.get_property(id, "")
    }

    pub fn set_property_by_string(&mut self, id: &str, value: &str) -> Result<()> {
        self.properties.set_property_by_string(id, value)
    }

    pub fn get_property_by_string(&self, id: &str) -> Result<String> {
        self.properties.get_property_by_string(id, "")
    }
}
