import {
  getAllContractsAndAccount,
} from '../contracts/compiled';
import { getItemEvents, getPlayers } from '../contracts/events';

export const appBoot = async () => {
  const {
    contracts,
    account
  } =  await getAllContractsAndAccount();
  const itemEvents = await getItemEvents();
  const players = await getPlayers();
  return {
    contracts,
    account,
    itemEvents,
    players,
  };
};