
import { getAllContracts } from './contracts';

export const getAccount = () => window.ethereum.selectedAddress;

export const getAllContractsAndAccount = async () => {
  const contracts = await getAllContracts();
  const account = getAccount();
  return {
    account,
    contracts,
  }
};

