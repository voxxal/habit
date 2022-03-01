import dayjs from "dayjs";
import { useEffect, useState } from "react";
import Home from "./pages/Home";
import Streak from "./pages/Streak";
import { State, StreakData } from "./state";
import { Route, Routes, useLocation } from "react-router-dom";
import { nanoid } from "nanoid";

declare global {
  interface Crypto {
    randomUUID: () => string;
  }
}

function App() {
  const location = useLocation();
  const [state, setState] = useState<State>(() =>
    Object.assign(
      {
        userId: nanoid(), //TODO temp solution
        experience: 0,
        level: 1,
        streaks: [],
      },
      JSON.parse(localStorage.getItem("habitsSave") || "{}")
    )
  );

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
      <Route
        path="/streak/:streakId"
        element={<Streak state={state} setState={setState} />}
      />
    </Routes>
  );
}

export default App;
