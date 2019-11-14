cpp!{{
    #include "Arp/Device/Interface/Services/IDeviceStatusService.hpp"
    #include "Arp/Device/Interface/Services/IDeviceInfoService.hpp"
    #include "Arp/System/Rsc/ServiceManager.hpp"
    #include "Arp/System/Rsc/Services/RscVariant.hxx"

    using namespace Arp::System::Rsc;
    using namespace Arp::Device::Interface::Services;
}}

cpp_class!(pub unsafe struct DeviceStatusService as "IDeviceStatusService::Ptr");

impl DeviceStatusService {
    pub fn get_service() -> Self {
        cpp!(unsafe [] -> DeviceStatusService as "IDeviceStatusService::Ptr" {
            return ServiceManager::GetService<IDeviceStatusService>();
        })
    }

    pub fn cpu_load(&self) -> i8 {
        cpp!(unsafe [self as "const IDeviceStatusService::Ptr*"] -> i8 as "unsigned char" {
            RscVariant<512> m_rscCpuLoad;
            byte m_byCpuLoad;

            if (self != NULL)
            {
                // get some values from device status service (dynamic data)
                m_rscCpuLoad = self->get()->GetItem("Status.Cpu.0.Load.Percent");
                m_rscCpuLoad.CopyTo(m_byCpuLoad);
            }

            return m_byCpuLoad;
        })
    }
}
