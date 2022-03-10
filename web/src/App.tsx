import dayjs from "dayjs";
import { useEffect, useState, useReducer } from "react";
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

function App() {
  const [state, dispatch] = useReducer(stateReducer, initalState);
  //@ts-ignore
  window.state = state;
  useEffect(() => {
    let data = localStorage.getItem("habitsSave");
    if (!data) {
      console.log("Can't fetch data from localStorage");
      return;
    }

    if (JSON.parse(data)) {
      dispatch({ type: ActionType.FetchData, payload: JSON.parse(data) });
      return;
    }
    console.log("Failed to parse data");
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      dispatch({ type: ActionType.UpdateStreaks });
    }, 1000 * 60 * 5);
    return () => clearInterval(interval);
  });

  useEffect(() => {
    localStorage.setItem("habitsSave", JSON.stringify(state));
  }, [state]);

  return (
    <StateContext.Provider value={{ state, dispatch }}>
      <Routes>
        <Route path="/" element={<HomePage />} />
        {/* <Route
          path="/tile/:tileId"
          element={<TilePage state={state} setState={setState} />}
        /> */}
      </Routes>
    </StateContext.Provider>
  );
}

export default App;
