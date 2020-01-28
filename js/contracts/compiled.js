import { getAllContracts } from './contracts';
import { getWeb3 } from './web3';

export const getAccount = async () => {
  const web3 = await getWeb3();
  return web3.utils.toChecksumAddress(window.ethereum.selectedAddress);
};

export const getAllContractsAndAccount = async () => {
  const contracts = await getAllContracts();
  const account = await getAccount();
  return {
    account,
    contracts,
  }
};

