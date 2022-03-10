import dayjs from "dayjs";
import { TileData } from "../state";
import { MouseEventHandler } from "react";

function CompleteButton({
  data,
  complete,
}: {
  data: TileData;
  complete: MouseEventHandler<HTMLButtonElement>;
}) {
  return (
    <>
      {dayjs().isSame(data.lastCheck, "day") ? (
        <button className="mt-auto h-10 w-full cursor-default items-center rounded-full border-2 border-dashed border-green-500 font-semibold text-green-500">
          <svg
            className="m-auto block h-6 w-6 fill-green-500"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            x="16"
            y="16"
            viewBox="0 0 32 32"
            xmlSpace="preserve"
          >
            <path d="M16,0C7.164,0,0,7.164,0,16s7.164,16,16,16s16-7.164,16-16S24.836,0,16,0z M13.52,23.383    L6.158,16.02l2.828-2.828l4.533,4.535l9.617-9.617l2.828,2.828L13.52,23.383z" />
          </svg>
        </button>
      ) : (
        <button
          className="mt-auto h-10 w-full rounded-full bg-green-500 font-semibold text-white transition-all hover:scale-110 hover:bg-green-600"
          onClick={complete}
        >
          Complete
        </button>
      )}
    </>
  );
}

export default CompleteButton;
