import React, { useReducer, createContext } from "react";
import { nanoid } from "nanoid";
import dayjs, { Dayjs } from "dayjs";
import { nextLevelTotalExp } from "./util";

interface TileData {
  // TODO discription
  id: string;
  name: string;
  streak: number;
  startTime: Dayjs;
  lastCheck: Dayjs;
}
// TODO make this a "Data" interface and include is loading/error
interface State {
  userId: string;
  experience: number;
  level: number;
  tiles: TileData[];
}

enum ActionType {
  FetchData,
  UpdateStreaks,
  CreateTile,
  DeleteTile,
  RenameTile,
  CompleteTile,
}

type Action =
  | {
      type: ActionType.FetchData;
      payload: State;
    }
  | {
      type: ActionType.UpdateStreaks;
    }
  | {
      type: ActionType.CreateTile;
      payload: string;
    }
  | {
      type: ActionType.DeleteTile;
      payload: TileData;
    }
  | {
      type: ActionType.RenameTile;
      payload: { data: TileData; rename: string };
    }
  | {
      type: ActionType.CompleteTile;
      payload: TileData;
    };

const initalState: State = {
  userId: nanoid(),
  experience: 0,
  level: 1,
  tiles: [],
};
// TODO stop making shallow copies
const stateReducer = (state: State, action: Action) => {
  switch (action.type) {
    case ActionType.FetchData: {
      return { ...initalState, ...action.payload };
    }
    case ActionType.UpdateStreaks: {
      return {
        ...state,
        tiles: state.tiles.map((tile) => {
          const newTiles: TileData = { ...tile };
          if (dayjs().diff(tile.lastCheck, "day") >= 2) {
            newTiles.streak = 0;
          }
          return newTiles;
        }),
      };
    }
    case ActionType.CreateTile: {
      return {
        ...state,
        tiles: [
          ...state.tiles,
          {
            id: nanoid(),
            name: "",
            streak: 0,
            startTime: dayjs(),
            lastCheck: dayjs().subtract(1, "day"),
          },
        ],
      };
    }
    case ActionType.DeleteTile: {
      return {
        ...state,
        tiles: state.tiles.filter((tile) => tile.id !== action.payload.id),
      };
    }
    case ActionType.RenameTile: {
      const oldData = state.tiles.find(
        (tile) => tile.id === action.payload.data.id
      );
      if (!oldData) return state;
      oldData.name = action.payload.rename;
      return {
        ...state,
        tiles: state.tiles,
      };
    }
    case ActionType.CompleteTile: {
      //TODO very round about just map it into a new tile array
      const newState = structuredClone(state) as State;
      const newData = newState.tiles.find(
        (tile) => tile.id === action.payload.id
      );
      if (!newData) return newState;
      newData.streak += 1;
      newData.lastCheck = dayjs();
      const exp = newState.experience + 1000 * (2 / newData.streak);
      let levelUp = 0;
      while (exp >= nextLevelTotalExp(newState.level + levelUp)) levelUp++;
      debugger;
      return {
        ...newState,
        level: newState.level + levelUp,
        experience: Math.floor(exp),
        tiles: newState.tiles,
      };
    }
    default:
      return state;
  }
};
// TODO 3rd argument is a dynamic initalState so you might need to put this into the App component later.

const StateContext = createContext<{
  state: State;
  dispatch: React.Dispatch<Action>;
}>({
  state: initalState,
  dispatch: () => undefined,
});

export { initalState, stateReducer, StateContext, ActionType };
export type { TileData, State, Action };
