use crate::stm32::ETHERNET_MAC;

mod miim;
pub use miim::*;

/// Ethernet media access control (MAC).
pub struct EthernetMAC {
    pub(crate) eth_mac: ETHERNET_MAC,
}

impl EthernetMAC {
    /// Create a new EthernetMAC that does not own its MDIO and MDC pins.
    pub fn new(eth_mac: ETHERNET_MAC) -> Self {
        Self { eth_mac }
    }

    /// Borrow access to the MAC's SMI.
    ///
    /// Allows for controlling and monitoring any PHYs that may be accessible via the MDIO/MDC
    /// pins.
    ///
    /// Exclusive access to the `MDIO` and `MDC` is required to ensure that are not used elsewhere
    /// for the duration of SMI communication.
    pub fn smi<'eth, 'pins, Mdio, Mdc>(
        &'eth mut self,
        mdio: &'pins mut Mdio,
        mdc: &'pins mut Mdc,
    ) -> Stm32Miim<'eth, 'pins, Mdio, Mdc>
    where
        Mdio: MdioPin,
        Mdc: MdcPin,
    {
        Stm32Miim::new(&self.eth_mac.macmiiar, &self.eth_mac.macmiidr, mdio, mdc)
    }

    /// Turn this [`EthernetMAC`] into an [`EthernetMACWithSmi`]
    pub fn with_smi<MDIO, MDC>(self, mdio: MDIO, mdc: MDC) -> EthernetMACWithMiim<MDIO, MDC>
    where
        MDIO: MdioPin,
        MDC: MdcPin,
    {
        EthernetMACWithMiim {
            eth_mac: self.eth_mac,
            mdio,
            mdc,
        }
    }
}

/// Ethernet media access control (MAC) with owned SMI
///
/// This version of the struct owns it's SMI pins,
/// allowing it to be used directly, instead of requiring
/// that a  [`Miim`] is created.
pub struct EthernetMACWithMiim<MDIO, MDC>
where
    MDIO: MdioPin,
    MDC: MdcPin,
{
    pub(crate) eth_mac: ETHERNET_MAC,
    mdio: MDIO,
    mdc: MDC,
}

impl<MDIO, MDC> EthernetMACWithMiim<MDIO, MDC>
where
    MDIO: MdioPin,
    MDC: MdcPin,
{
    /// Create a new EthernetMAC with owned MDIO and MDC pins.
    ///
    /// To interact with a connected Phy, use this struct's impl of
    /// [`SerialManagement`]
    pub fn new(eth_mac: ETHERNET_MAC, mdio: MDIO, mdc: MDC) -> Self {
        Self { eth_mac, mdio, mdc }
    }

    /// Release the owned MDIO and MDC pins, and return an EthernetMAC that
    /// has to borrow the MDIO and MDC pins.
    pub fn release_pins(self) -> (EthernetMAC, MDIO, MDC) {
        (
            EthernetMAC {
                eth_mac: self.eth_mac,
            },
            self.mdio,
            self.mdc,
        )
    }
}

impl<MDIO, MDC> EthernetMACWithMiim<MDIO, MDC>
where
    MDIO: MdioPin,
    MDC: MdcPin,
{
    pub fn read(&mut self, phy: u8, reg: u8) -> u16 {
        miim_read(&self.eth_mac.macmiiar, &self.eth_mac.macmiidr, phy, reg)
    }

    pub fn write(&mut self, phy: u8, reg: u8, data: u16) {
        miim_write(
            &self.eth_mac.macmiiar,
            &self.eth_mac.macmiidr,
            phy,
            reg,
            data,
        )
    }
}

#[cfg(feature = "ieee802_3_miim")]
impl<MDIO, MDC> miim::Miim for EthernetMACWithMiim<MDIO, MDC>
where
    MDIO: MdioPin,
    MDC: MdcPin,
{
    fn read(&mut self, phy: u8, reg: u8) -> u16 {
        miim_read(&self.eth_mac.macmiiar, &self.eth_mac.macmiidr, phy, reg)
    }

    fn write(&mut self, phy: u8, reg: u8, data: u16) {
        miim_write(
            &self.eth_mac.macmiiar,
            &self.eth_mac.macmiidr,
            phy,
            reg,
            data,
        )
    }
}
