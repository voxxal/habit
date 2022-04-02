import dayjs from "dayjs";
import localForage from "localforage";
import React, { useEffect, useState, useReducer } from "react";
import HomePage from "./pages/Home";
import TilePage from "./pages/Tile";
import {
  State,
  TileData,
  StateContext,
  stateReducer,
  initalState,
  ActionType,
} from "./state";
import { Route, Routes } from "react-router-dom";
import { sync } from "./api";
import { StreakBitSet } from "./streakbitset";
import { nanoid } from "nanoid";

localForage.config({
  name: "habit",
});

function App() {
  const [state, dispatch] = useReducer(stateReducer, initalState);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    const fetchDataFromLocalStorage = async () => {
      //TODO extract into another function
      const data = await localForage.getItem<string>("habitsSave") ?? JSON.stringify({userId: nanoid(), experience: 0, level: 1, tiles: []});
      const state = JSON.parse(data);
      if (state) {
        state.tiles = state.tiles.map((tile: TileData) => {
          tile.streak = new StreakBitSet(tile.streak._bit_capacity, tile.streak._buffer);
          return tile;
        });
        dispatch({ type: ActionType.FetchData, payload: state });
        setLoading(false);
        return;
      }
      console.log("Failed to parse data");
    };

    fetchDataFromLocalStorage();
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      dispatch({ type: ActionType.UpdateStreaks });
      sync(state);
    }, 1000 * 60 * 5);
    return () => clearInterval(interval);
  });

  useEffect(() => {
    localForage.setItem("habitsSave", JSON.stringify(state));
  }, [state]);

  // loading screen forever when first load
  return (
    <StateContext.Provider value={{ state, dispatch }}>
      {loading ? (
        <div className="flex h-screen items-center justify-center text-2xl">
          Loading...
        </div>
      ) : (
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/tile/:tileId" element={<TilePage />} />
        </Routes>
      )}
    </StateContext.Provider>
  );
}

export default App;
