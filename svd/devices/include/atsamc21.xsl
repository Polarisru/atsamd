<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The DMAC trigger sources in the original SVD only have the 0=disabled
  enumeration value -->
  <xsl:template match="/device/peripherals/peripheral[name='DMAC']/registers/register[name='CHCTRLB']/fields/field[name='TRIGSRC']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <xsl:copy-of select="./enumeratedValue"/>
      <enumeratedValue>
        <name>TSENS</name>
        <description>TSENS Result Ready Trigger</description>
        <value>0x01</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_RX</name>
        <description>SERCOM0 RX Trigger</description>
        <value>0x02</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_TX</name>
        <description>SERCOM0 TX Trigger</description>
        <value>0x03</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_RX</name>
        <description>SERCOM1 RX Trigger</description>
        <value>0x04</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_TX</name>
        <description>SERCOM1 TX Trigger</description>
        <value>0x05</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_RX</name>
        <description>SERCOM2 RX Trigger</description>
        <value>0x06</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_TX</name>
        <description>SERCOM2 TX Trigger</description>
        <value>0x07</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_RX</name>
        <description>SERCOM3 RX Trigger</description>
        <value>0x08</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_TX</name>
        <description>SERCOM3 TX Trigger</description>
        <value>0x09</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_RX</name>
        <description>SERCOM4 RX Trigger</description>
        <value>0x0A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_TX</name>
        <description>SERCOM4 TX Trigger</description>
        <value>0x0B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_RX</name>
        <description>SERCOM5 RX Trigger</description>
        <value>0x0C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_TX</name>
        <description>SERCOM5 TX Trigger</description>
        <value>0x0D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>CAN0_DEBUG</name>
        <description>CAN0 Debug Trigger</description>
        <value>0x0E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>CAN1_DEBUG</name>
        <description>CAN1 Debug Trigger</description>
        <value>0x0F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_OVF</name>
        <description>TCC0 Overflow Trigger</description>
        <value>0x10</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC0</name>
        <description>TCC0 Match/Compare 0 Trigger</description>
        <value>0x11</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC1</name>
        <description>TCC0 Match/Compare 1 Trigger</description>
        <value>0x12</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC2</name>
        <description>TCC0 Match/Compare 2 Trigger</description>
        <value>0x13</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC3</name>
        <description>TCC0 Match/Compare 3 Trigger</description>
        <value>0x14</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_OVF</name>
        <description>TCC1 Overflow Trigger</description>
        <value>0x15</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC0</name>
        <description>TCC1 Match/Compare 0 Trigger</description>
        <value>0x16</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC1</name>
        <description>TCC1 Match/Compare 1 Trigger</description>
        <value>0x17</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_OVF</name>
        <description>TCC2 Overflow Trigger</description>
        <value>0x18</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC0</name>
        <description>TCC2 Match/Compare 0 Trigger</description>
        <value>0x19</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC1</name>
        <description>TCC2 Match/Compare 1 Trigger</description>
        <value>0x1A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_OVF</name>
        <description>TC0 Overflow Trigger</description>
        <value>0x1B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_MC0</name>
        <description>TC0 Match/Compare 0 Trigger</description>
        <value>0x1C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_MC1</name>
        <description>TC0 Match/Compare 1 Trigger</description>
        <value>0x1D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_OVF</name>
        <description>TC1 Overflow Trigger</description>
        <value>0x1E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC0</name>
        <description>TC1 Match/Compare 0 Trigger</description>
        <value>0x1F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC1</name>
        <description>TC1 Match/Compare 1 Trigger</description>
        <value>0x20</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_OVF</name>
        <description>TC2 Overflow Trigger</description>
        <value>0x21</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC0</name>
        <description>TC2 Match/Compare 0 Trigger</description>
        <value>0x22</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC1</name>
        <description>TC2 Match/Compare 1 Trigger</description>
        <value>0x23</value>
      </enumeratedValue>      
      <enumeratedValue>
        <name>TC3_OVF</name>
        <description>TC3 Overflow Trigger</description>
        <value>0x24</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC0</name>
        <description>TC3 Match/Compare 0 Trigger</description>
        <value>0x25</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC1</name>
        <description>TC3 Match/Compare 1 Trigger</description>
        <value>0x26</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_OVF</name>
        <description>TC4 Overflow Trigger</description>
        <value>0x27</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC0</name>
        <description>TC4 Match/Compare 0 Trigger</description>
        <value>0x28</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC1</name>
        <description>TC4 Match/Compare 1 Trigger</description>
        <value>0x29</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC0_RESRDY</name>
        <description>ADC0 Result Ready Trigger</description>
        <value>0x2A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC1_RESRDY</name>
        <description>ADC1 Result Ready Trigger</description>
        <value>0x2B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SDADC_RESRDY</name>
        <description>SDADC Result Ready Trigger</description>
        <value>0x2C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY</name>
        <description>DAC Empty Trigger</description>
        <value>0x2D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>PTC_EOC</name>
        <description>PTC End of Conversion Trigger</description>
        <value>0x2E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>PTC_WCOMP</name>
        <description>PTC Window Compare Trigger</description>
        <value>0x2F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>PTC_SEQ</name>
        <description>PTC Sequence Trigger</description>
        <value>0x30</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM6_RX</name>
        <description>SERCOM6 RX Trigger</description>
        <value>0x31</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM6_TX</name>
        <description>SERCOM6 TX Trigger</description>
        <value>0x32</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM7_RX</name>
        <description>SERCOM7 RX Trigger</description>
        <value>0x33</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM7_TX</name>
        <description>SERCOM7 TX Trigger</description>
        <value>0x34</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_OVF</name>
        <description>TC5 Overflow Trigger</description>
        <value>0x35</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC0</name>
        <description>TC5 Match/Compare 0 Trigger</description>
        <value>0x36</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC1</name>
        <description>TC5 Match/Compare 1 Trigger</description>
        <value>0x37</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_OVF</name>
        <description>TC6 Overflow Trigger</description>
        <value>0x38</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC0</name>
        <description>TC6 Match/Compare 0 Trigger</description>
        <value>0x39</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC1</name>
        <description>TC6 Match/Compare 1 Trigger</description>
        <value>0x3A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_OVF</name>
        <description>TC7 Overflow Trigger</description>
        <value>0x3B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC0</name>
        <description>TC7 Match/Compare 0 Trigger</description>
        <value>0x3C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC1</name>
        <description>TC7 Match/Compare 1 Trigger</description>
        <value>0x3D</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- Some register bits are erroneously write-only in the
  original SVD, remove the access field that marks it as such -->
  <xsl:template match="/device/peripherals/peripheral[name='PORT']/registers/register[name='PINCFG0_%s']/fields/field[name='DRVSTR']/access" />
  <xsl:template match="/device/peripherals/peripheral[name='RTC']/registers/cluster/register[name='CTRL']/fields/field[name='SWRST']/access" />

</xsl:stylesheet>
