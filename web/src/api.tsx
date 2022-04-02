import ky from "ky";
import { State } from "./state";

const sync = async (state: State) => ky.post("http://localhost:8080/sync", {json: state})

export {
    sync
}