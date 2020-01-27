import Battle from "../../../kitty-wars/build/contracts/Battle.json";
import KittyOwnership from '../../../kitty-wars/build/contracts/KittyOwnership.json';
import ItemOwnership from '../../../kitty-wars/build/contracts/ItemOwnership.json';
import PlayerRepo from '../../../kitty-wars/build/contracts/PlayerRepo.json';
import Proxy from '../../../kitty-wars/build/contracts/Proxy.json';

import { getWeb3 } from './web3';
import {
  battleContractName,
  itemOwnershipContractName,
  playerRepoContractName,
  proxyContractName
} from './contract-names';


const allContracts = {
  [battleContractName]: Battle,
  KittyOwnership,
  [itemOwnershipContractName]: ItemOwnership,
  [playerRepoContractName]: PlayerRepo,
  [proxyContractName]: Proxy,
};

export const getAllContracts = async () => {
  const web3 = await getWeb3();
  const networkId = await web3.eth.net.getId();
  return Object.entries(allContracts)
    .reduce(
      (
        acc,
        [key, value]
      ) => ({
        ...acc,
        [key]: new web3.eth.Contract(
          value.abi,
          value.networks[networkId] && value.networks[networkId].address,
        ),
      }), {}
    );
};