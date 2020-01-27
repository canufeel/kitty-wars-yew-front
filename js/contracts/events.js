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
  const players = [];
  if (events.length) {
    for (event in events) {
      const [
        weaponId,
        armorId,
        kittyId,
      ] = await playerRepo.getPlayer(event.returnValues.playerAddress);
      players.push({
        weaponId,
        armorId,
        kittyId,
      });
    }
  }
  return players;
};