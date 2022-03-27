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

localForage.config({
  name: "habit",
});

function App() {
  const [state, dispatch] = useReducer(stateReducer, initalState);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    const fetchDataFromLocalStorage = async () => {
      //TODO extract into another function
      const data = await localForage.getItem<string>("habitsSave");
      if (!data) {
        console.log("Can't fetch data from localStorage");
        return;
      }
      if (JSON.parse(data)) {
        dispatch({ type: ActionType.FetchData, payload: JSON.parse(data) });
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
