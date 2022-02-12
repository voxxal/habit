import { Dayjs } from "dayjs";

interface StreakData {
  id: number;
  name: string;
  streak: number;
  startTime: Dayjs;
  lastCheck: Dayjs;
}

interface State {
  userId: number;
  experience: number;
  level: number;
  streaks: StreakData[];
}

export type { StreakData, State };
