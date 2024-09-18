#![warn(missing_docs)]
use std::{collections::HashMap, time::Duration};

use generic_camera::{GenCam, GenCamCtrl, GenCamError, GenCamResult, Property};
use refimage::GenericImage;

use crate::asihandle::AsiHandle;

/// Generic camera control for ASI cameras.
#[derive(Debug)]
pub struct GenCamAsi {
    handle: AsiHandle,
    caps: HashMap<GenCamCtrl, Property>,
}

impl GenCam for GenCamAsi {
    fn set_exposure(&mut self, exposure: std::time::Duration) -> GenCamResult<Duration> {
        self.handle.set_exposure(exposure)?;
        self.handle.get_exposure()
    }

    fn start_exposure(&mut self) -> GenCamResult<()> {
        self.handle.start_exposure()
    }

    fn image_ready(&self) -> GenCamResult<bool> {
        self.handle.image_ready()
    }

    fn download_image(&mut self) -> GenCamResult<GenericImage> {
        self.handle.download_image()
    }
    
    fn info_handle(&self) -> Option<generic_camera::AnyGenCamInfo> {
        todo!()
    }
    
    fn vendor(&self) -> &str {
        todo!()
    }
    
    fn camera_ready(&self) -> bool {
        todo!()
    }
    
    fn camera_name(&self) -> &str {
        self.handle.camera_name()
    }
    
    fn list_properties(&self) -> &std::collections::HashMap<GenCamCtrl, generic_camera::Property> {
        todo!()
    }
    
    fn get_property(&self, name: GenCamCtrl) -> GenCamResult<(&generic_camera::PropertyValue, bool)> {
        todo!()
    }
    
    fn set_property(
        &mut self,
        name: GenCamCtrl,
        value: &generic_camera::PropertyValue,
        auto: bool,
    ) -> GenCamResult<()> {
        todo!()
    }
    
    fn cancel_capture(&self) -> GenCamResult<()> {
        todo!()
    }
    
    fn is_capturing(&self) -> bool {
        todo!()
    }
    
    fn capture(&self) -> GenCamResult<GenericImage> {
        todo!()
    }
    
    fn camera_state(&self) -> GenCamResult<generic_camera::GenCamState> {
        todo!()
    }
    
    fn get_exposure(&self) -> GenCamResult<std::time::Duration> {
        todo!()
    }
    
    fn set_roi(&mut self, roi: &generic_camera::GenCamRoi) -> GenCamResult<&generic_camera::GenCamRoi> {
        todo!()
    }
    
    fn get_roi(&self) -> &generic_camera::GenCamRoi {
        todo!()
    }
}