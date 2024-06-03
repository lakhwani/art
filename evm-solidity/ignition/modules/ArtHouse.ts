import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const DEFAULT_ROYALTY_RATE = 10;

const ArtHouseModule = buildModule("ArtHouseModule", (m) => {
  const royaltyRate = m.getParameter("royaltyRate", DEFAULT_ROYALTY_RATE);

  const artHouseBase = m.contract("ArtHouseBase", [royaltyRate]);

  return { artHouseBase };
});

export default ArtHouseModule;
