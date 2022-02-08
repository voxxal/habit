import dayjs from "dayjs";
import { useEffect, useState } from "react";
import Streak from "./components/Streak";
import Home from "./Home";
import { State, StreakData } from "./state";

function App() {
  let [state, setState] = useState<State>({
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



  return (<Home state={state} setState={setState} />)
}

export default App;
