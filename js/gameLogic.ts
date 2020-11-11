import * as $ from "jquery";
import Cookies from "js-cookie";
import { speed } from "./index";

class Point {
  constructor(public x: number, public y: number) {}
}

export const GRID_SIZE = 20;

export const startGame = () => {
  import("../pkg/index").then((wasm) => {
    let direction = "";
    let score = 0;

    document.addEventListener("keydown", (event) => {
      if (!(event.isComposing || event.keyCode === 229)) {
        let key = event.key.toLowerCase();
        if ((key === "w" || key === "arrowup") && direction !== "down") {
          direction = "up";
        } else if ((key === "s" || key === "arrowdown") && direction !== "up") {
          direction = "down";
        } else if (
          (key === "a" || key === "arrowleft") &&
          direction !== "right"
        ) {
          direction = "left";
        } else if (
          (key === "d" || key === "arrowright") &&
          direction !== "left"
        ) {
          direction = "right";
        }
      }
    });

    let snake = [new Point(0, 0)];
    let fruit = new Point(wasm.ranint(GRID_SIZE), wasm.ranint(GRID_SIZE));

    let hs =
      Cookies.get("hs" + speed) === undefined ? 0 : Cookies.get("hs" + speed);

    // let hs = document.cookie.split(";").map((a) => a.startsWith("hs" + speed + "=") ? a.replace("hs" + speed + "=", "") : "").join("");

    $("#score").html("Score " + score);
    $("#hs").html("High Score " + hs);

    let interval = setInterval(() => {
      let result = JSON.parse(
        wasm.update_wasm(
          JSON.stringify(snake),
          JSON.stringify(fruit),
          GRID_SIZE,
          direction
        )
      );
      if (result["failed"]) {
        $(".game").addClass("over");
        console.log("You failed");
        clearInterval(interval);
      }

      snake = result["snake"];
      fruit = result["fruit"];

      if (result["got_fruit"]) {
        $("#score").html("Score " + score);
        $("#hs").html("High Score " + hs);

        score++;
        hs = Math.max(+hs, score);

        Cookies.set("hs" + speed, "" + hs, { expires: 1000000 });
      }

      $(".snake").removeClass("snake");
      $(".fruit").removeClass("fruit");

      $("#" + (fruit["x"] * GRID_SIZE + fruit["y"])).addClass("fruit");

      for (let a in snake) {
        let pos = snake[a];
        $("#" + (pos["x"] * GRID_SIZE + pos["y"])).addClass("snake");
      }
    }, speed);
  });
};
