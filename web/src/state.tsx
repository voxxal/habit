import { Dayjs } from "dayjs";

interface StreakData {
  id: string;
  name: string;
  streak: number;
  startTime: Dayjs;
  lastCheck: Dayjs;
}

interface State {
  userId: string;
  experience: number;
  level: number;
  streaks: StreakData[];
}

export type { StreakData, State };
