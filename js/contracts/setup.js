import { getAccount } from './compiled';
import { getAllContracts } from './contracts';
import { proxyContractName } from './contract-names';

const max = 9;

const randomInt = (_max) => {
  return Math.floor(Math.random() * Math.floor(_max));
};

export const joinAndLoot = async () => {
  const account = getAccount();
  const weaponPower = randomInt(max);
  const armorPower = randomInt(max);
  const {
    [proxyContractName]: proxy,
  } = await getAllContracts();
  return proxy.methods.newPlayer(
    weaponPower,
    armorPower,
  ).send({
    from: account,
  })
};
