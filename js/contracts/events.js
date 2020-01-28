import { getAllContracts } from './contracts';
import { itemOwnershipContractName, playerRepoContractName } from './contract-names';


export const getItemEvents = async () => {
  const {
    [itemOwnershipContractName]: itemOwnership,
  } = await getAllContracts();
  const events = await itemOwnership.getPastEvents('ItemForged');
  return events.map(evt => evt.returnValues);
};

export const getPlayers = async () => {
  const {
    [playerRepoContractName]: playerRepo,
  } = await getAllContracts();
  const events = await playerRepo.getPastEvents('PlayerAdded');
  const playersHash = {};
  if (events.length) {
    for (event of events) {
      const {
        returnValues: {
          playerAddress,
        }
      } = event;
      const {
        weaponId,
        armorId,
        kittyId,
      } = await playerRepo.methods.getPlayer(playerAddress).call();
      playersHash[playerAddress] = {
        playerAddress,
        weaponId,
        armorId,
        kittyId,
      };
    }
  }
  return playersHash;
};