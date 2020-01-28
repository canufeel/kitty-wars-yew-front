import { getAllContracts } from './contracts';
import {
  battleContractName,
  itemOwnershipContractName,
  playerRepoContractName,
} from './contract-names';

const playerCommitmentPropNames = ['playerOneCommitmentSent', 'playerTwoCommitmentSent'];

export const getBattles = async () => {
  const {
    [battleContractName]: battle,
  } = await getAllContracts();
  const battlesCreated = await battle.getPastEvents('BattleCreated');
  const battlesHash = {};
  battlesCreated.forEach(({
    returnValues: {
      player,
      battleId,
    }
  }) => {
    battlesHash[battleId] = {
      playerOne: player,
      battleId: parseInt(battleId, 10),
      playerTwo: null,
      started: false,
      playerOneCommitmentSent: false,
      playerTwoCommitmentSent: false,
      winner: null,
      loser: null,
      damageWinner: null,
      damageLoser: null,
      finished: false,
    };
  });
  const battlesJoined = await battle.getPastEvents('BattleJoined');
  battlesJoined.forEach(({
    returnValues: {
      player,
      battleId,
    }
  }) => {
    battlesHash[battleId] = {
      ...battlesHash[battleId],
      playerTwo: player,
      started: true,
    };
  });
  const battlesCommitments = await battle.getPastEvents('BattleCommitmentsSent');
  battlesCommitments.forEach(({
    returnValues: {
      player,
      battleId,
    }
  }) => {
    const isPlayerOne = player === battlesHash[battleId].playerOne;
    const propName = isPlayerOne ? playerCommitmentPropNames[0] : playerCommitmentPropNames[1];
    const canResolve = isPlayerOne ? !!battlesHash[battleId][playerCommitmentPropNames[1]] : !!battlesHash[battleId][playerCommitmentPropNames[0]];
    battlesHash[battleId] = {
      ...battlesHash[battleId],
      [propName]: true,
      canResolve,
    };
  });
  const battlesFinished = await battle.getPastEvents('BattleWon');
  battlesFinished.forEach(({
    returnValues: {
      winner,
      loser,
      damageWinner,
      damageLoser,
      battleId,
    }
  }) => {
    battlesHash[battleId] = {
      ...battlesHash[battleId],
      finished: true,
      winner,
      loser,
      damageWinner,
      damageLoser,
    };
  });
  const isBattlingHash = {};
  Object.values(battlesHash).forEach(
    (
      {
        playerOne,
        playerTwo,
        finished,
      }
    ) => {
      if (!finished) {
        isBattlingHash[playerOne] = true;
        if (!!playerTwo) {
          isBattlingHash[playerTwo] = true;
        }
      }
    }
  );
  return {
    battles: battlesHash,
    isBattling: isBattlingHash,
  };
};

export const getItemEvents = async () => {
  const {
    [itemOwnershipContractName]: itemOwnership,
  } = await getAllContracts();
  const events = await itemOwnership.getPastEvents('ItemForged');
  const eventsHash = {};
  events.forEach(
    (
      {
        returnValues: {
          itemId,
          itemType,
          itemPower,
        }
      }
    ) => {
      eventsHash[itemId] = {
        itemId,
        itemType,
        itemPower,
      };
    }
  );
  return eventsHash;
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