import {
  getAllContractsAndAccount,
} from '../contracts/compiled';
import {
  getBattles,
  getItemEvents,
  getPlayers,
} from '../contracts/events';

export const appBoot = async () => {
  const {
    account
  } =  await getAllContractsAndAccount();
  const items = await getItemEvents();
  const players = await getPlayers();
  const {
    battles,
    isBattling,
  } = await getBattles();
  return {
    battles,
    account,
    items,
    players,
    isBattling,
  };
};