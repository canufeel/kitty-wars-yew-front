import { getAccount } from './compiled';
import { getAllContracts } from './contracts';
import { proxyContractName } from './contract-names';

const max = 9;

const randomInt = (_max) => {
  return Math.floor(Math.random() * Math.floor(_max));
};

export const join = async () => {
  const account = getAccount();
  const {
    [proxyContractName]: proxy,
  } = await getAllContracts();
  return proxy.methods.join().send({
    from: account,
  })
};

export const loot = async () => {
  const weaponPower = randomInt(max);
  const armorPower = randomInt(max);
  const account = getAccount();
  const {
    [proxyContractName]: proxy,
  } = await getAllContracts();
  return proxy.methods.loot(weaponPower, armorPower).send({
    from: account,
  });
};
