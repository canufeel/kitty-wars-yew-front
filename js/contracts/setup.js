import { getAccount } from './compiled';
import { getAllContracts } from './contracts';
import {
  proxyContractName,
  battleContractName,
} from './contract-names';

const max = 9;

const randomInt = (_max) => {
  return Math.floor(Math.random() * Math.floor(_max));
};

export const joinAndLoot = async () => {
  const account = await getAccount();
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

const _createBattle = async (battleId = 0) => {
  const {
    [battleContractName]: battle,
  } = await getAllContracts();
  const account = await getAccount();
  await battle.startBattle(battleId).send({
    from: account,
  });
};

export const createBattle = async () => {
  return _createBattle();
};

export const joinBattle = async (battleId) => {
  return _createBattle(battleId);
};