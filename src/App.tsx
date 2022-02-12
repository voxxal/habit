import dayjs from "dayjs";
import { useEffect, useState } from "react";
import Home from "./Home";
import { uid } from "./util";
import { State, StreakData } from "./state";
import { Route, Routes } from "react-router-dom";

function App() {
  let [state, setState] = useState<State>({
    userId: uid.getUniqueID(),
    experience: 0,
    level: 1,
    streaks: [],
  });

  useEffect(() => {
    setState(
      Object.assign(
        { ...state },
        JSON.parse(localStorage.getItem("habitsSave") || "{}")
      )
    );
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setState({
        ...state,
        streaks: state.streaks.map((streak) => {
          let newStreak: StreakData = { ...streak };
          if (dayjs().diff(streak.lastCheck, "day") >= 2) {
            newStreak.streak = 0;
          }
          return newStreak;
        }),
      });
    }, 1000 * 60 * 5);
    return () => clearInterval(interval);
  });

  useEffect(() => {
    localStorage.setItem("habitsSave", JSON.stringify(state));
  }, [state]);

  return (
    <Routes>
      <Route path="/" element={<Home state={state} setState={setState} />} />
    </Routes>
  );
}

export default App;
