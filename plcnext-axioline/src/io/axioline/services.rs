#[macro_use]
extern crate cpp;

mod error;

cpp!{{
    #include "Arp/Io/Axioline/Services/IAxioMasterService.hpp"
    #include "Arp/Io/Axioline/Services/IAcyclicCommunicationService.hpp"
    #include "Arp/System/Rsc/ServiceManager.hpp"

    using namespace Arp::System::Rsc;
    using namespace Arp::Io::Axioline::Services;
}}

#[repr(C)]
pub struct PdiParam {
    pub slot: u16,
    pub subslot: u8,
    pub index: u16,
    pub subindex: u8,
}

#[repr(C)]
struct PdiError {
    error_code: u16,
    add_info: u16,
}

cpp_class!(pub unsafe struct AxioMasterService as "IAxioMasterService::Ptr");
cpp_class!(pub unsafe struct AcyclicCommunicationService as "IAcyclicCommunicationService::Ptr");

// TODO: Add error codes.

impl AxioMasterService {
    pub fn get_service() -> Self {
        cpp!(unsafe [] -> AxioMasterService as "IAxioMasterService::Ptr" {
            return ServiceManager::GetService<IAxioMasterService>();
        })
    }

    // TODO: Change the return value to the "response"
    pub fn axio_control(&self, request: Vec<u16>, response: &mut Vec<u16>) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     request as "std::vector<uint16>",
                     response as "std::vector<uint16>*"] -> u16 as "uint16" {

            uint16 m_Result;

            uint16 element = request[0];

            (*response)[0] = 1234;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->AxioControl(request, *response);
            }

            return m_Result;
        })
    }

    // This service stops the running of cycles and resets the driver. Output data is
    // disabled prior to this. The outputs respond as specified in the substitute value
    // behaviour. Once the service has been executed, the AXIObus master is in the Ready state.
    pub fn reset_master(&self) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*"] -> u16 as "uint16" {

            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->ResetMaster();
            }

            return m_Result.ErrorCode;
        })
    }

    // This service causes the AXIObus master to automatically generate a
    // configuration frame from the currently connected configuration and
    // to activate it in order to start the bus. Once the service has been
    // executed, the AXIObus master is in the Active state.
    // The new configuration frame is stored under the number specified in the
    // "frame" parameter. If there is already a configuration frame under this
    // number, this frame is overwritten.
    // The AXIObus master must be in the Ready state before this service is called.
    pub fn create_configuration(&self, frame: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     frame as "uint16"] -> u16 as "uint16" {

            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->CreateConfiguration(frame);
            }

            return m_Result.ErrorCode;
        })
    }

    // This service reads all entries of the configuration frame. 
    // TODO: Return the configuration frame.
    pub fn read_configuration(&self, frame: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     frame as "uint16"] -> u16 as "uint16" {

            std::vector<AxioDeviceConfiguration> configuration;
            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->ReadConfiguration(frame, configuration);
            }

            return m_Result.ErrorCode;
        })
    }

    // This service writes all entries of the configuration frame.
    // TODO: Provide the configuration frame.
    pub fn write_configuration(&self, frame: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     frame as "uint16"] -> u16 as "uint16" {

            std::vector<AxioDeviceConfiguration> configuration;
            //Push back new configuration created with default constructor.
            configuration.push_back(AxioDeviceConfiguration());
            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->WriteConfiguration(frame, configuration);
            }

            return m_Result.ErrorCode;
        })
    }

    // This service causes the AXIObus master to activate the specified configuration 
    // frame. In doing so, the loaded configuration frame is compared with the currently
    // connected bus configuration. If no errors are found, the AXIObus master activates
    // this configuration frame and switches to the Active state. 
    // The AXIObus master must be in the Ready state before the service is called and
    // a configuration frame must be loaded (e.g., using the “write_configuration” service.
    // If a configuration frame is already active, this must be deactivated before
    // executing this service using “deactivate_configuration”.
    pub fn activate_configuration(&self, frame: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     frame as "uint16"] -> u16 as "uint16" {

            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->ActivateConfiguration(frame);
            }

            return m_Result.ErrorCode;
        })
    }

    // This service deactivates the specified configuration frame. No further cycles are
    // run. Once the service has been executed, the AXIObus master is in the Ready state.
    // The specified configuration frame must not only exist, it must also be active when
    // the service is called.
    pub fn deactivate_configuration(&self, frame: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     frame as "uint16"] -> u16 as "uint16" {

            AxioResult m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->DeactivateConfiguration(frame);
            }

            return m_Result.ErrorCode;
        })
    }

    // This service loads the mapping for the internal DMA controller. Use this service to
    // assign the process data to the PD RAM interface.
    // The driver must be in the Active state before the service is called, i.e., an active
    // configuration frame is present. This is achieved by calling “create_configuration”, for example.
    // Note: The last loaded mapping is always used for process output data.
    pub fn load_pd_mapping(&self, direction: u16, relationship: u16, mode: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     direction as "uint16",
                     relationship as "uint16",
                     mode as "uint16"] -> u16 as "uint16" {

            const std::vector<uint16> request = {0x0728, 0x0004, direction, relationship, mode, 0x0000};
            std::vector<uint16> response;
            uint16 m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->AxioControl(request, response);
            }

            return m_Result;
        })
    }

    // This service enables the process data of the specified communication relationship (CR).
    // Once the service has been executed, the AXIObus master is in the Run state.
    // The AXIObus master must be in the Active state before the service is called, i.e.,
    // a configuration frame is present (“create_configuration” has already been called).
    // For the AXIObus master IP core without PD RAM interface, the processor interface must
    // be selected with the "relationship" parameter. 
    pub fn enable_output(&self, relationship: u16) -> u16 {
        cpp!(unsafe [self as "const IAxioMasterService::Ptr*",
                     relationship as "uint16"] -> u16 as "uint16" {

            const std::vector<uint16> request = {0x0701, 0x0001, relationship};
            std::vector<uint16> response;
            uint16 m_Result;

            if (self != NULL)
            {
                // response = request;
                m_Result = self->get()->AxioControl(request, response);
            }

            return m_Result;
        })
    }
}

impl AcyclicCommunicationService {
    pub fn get_service() -> Self {
        cpp!(unsafe [] -> AcyclicCommunicationService as "IAcyclicCommunicationService::Ptr" {
            return ServiceManager::GetService<IAcyclicCommunicationService>();
        })
    }

    // TODO: Provide the correct parameters
    pub fn pdi_read(&self, param: PdiParam, data: &mut Vec<u8>) -> u16 {
        cpp!(unsafe [self as "const IAcyclicCommunicationService::Ptr*",
                     param as "std::tuple<uint16,uint8,uint16,uint8>",
                     data as "std::vector<uint8>*"] -> u16 as "uint16" {

            PdiResult m_Result;
            PdiParam m_Param;
            m_Param.Slot = std::get<0>(param);
            m_Param.Subslot = std::get<1>(param);
            m_Param.Index = std::get<2>(param);
            m_Param.Subindex = std::get<3>(param);

            if (self != NULL)
            {
                m_Result = self->get()->PdiRead(m_Param, *data);
            }

            return m_Result.ErrorCode;
        })
    }
}
