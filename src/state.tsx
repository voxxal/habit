import { Dayjs } from "dayjs";

interface StreakData {
  name: string;
  streak: number;
  startTime: Dayjs;
  lastCheck: Dayjs;
}

interface State {
  experience: number;
  level: number;
  streaks: StreakData[];
}

export type { StreakData, State };
