import { Component } from '@angular/core';
import { HttpService } from './http.service';
import { interval, takeWhile, window } from 'rxjs';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title: String = "tic-tac-toe";
  updateBoard: Function;
  boardState: Array<Number> = [];
  makeManualMove: Function;
  gameState: Number = 0;
  gameOver = false;
  sideToMove: Number = 1;

  isManualMove: Function;
  isEngineMove: Function;
  playerX: Number = 2; // Engine
  playerY: Number = 1; // Manual
  updatePlayer: Function;

  resetBoard: Function;


  constructor(private http: HttpService) {
    this.http.postInit().subscribe((response) => {
      console.log(response);
    });

    this.updateBoard = function () {
      this.http.getState().subscribe((response) => {
        this.boardState = response;
        this.gameState = this.boardState[9];
        this.sideToMove = this.boardState[10];
        if (this.gameState !== 0) {
          this.gameOver = true;
        }
      });
    }
    
    this.makeManualMove = function (move: Number) {
      if (this.isManualMove() && !this.gameOver) {
        this.http.postManualMove(move).subscribe((response) => {
          console.log(response);
          this.updateBoard();
        })
      }
    }

    this.isManualMove = function () {
      return (this.sideToMove === 1 && this.playerX === 1) || (this.sideToMove === 2 && this.playerY === 1);
    }

    this.isEngineMove = function () {
      return (this.sideToMove === 1 && this.playerX === 2) || (this.sideToMove === 2 && this.playerY === 2);
    }


    interval(100).pipe(takeWhile(() => true)).subscribe(() => {
      if (this.isEngineMove() && !this.gameOver) {
        this.sideToMove = 0;
        this.http.postEngineMove().subscribe((response) => {
          console.log(response);
          this.updateBoard();
        })
      }
    })

    this.updatePlayer = function (player: Number, control: Number) {
      if (player === 1) {
        this.playerX = control;
      } else if (player === 2) {
        this.playerY = control;
      }
    }

    this.resetBoard = function () {
      this.http.postInit().subscribe((response) => {
        console.log(response);
        this.updateBoard();
        this.boardState = [];
        this.gameOver = false;
        this.sideToMove = 1;
        this.gameState = 0;
      })
    }
  }
}

