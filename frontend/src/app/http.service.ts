import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

const url = "http://127.0.0.1:8884/"

@Injectable({
  providedIn: 'root'
})
export class HttpService {
  constructor(private http: HttpClient) { }

  postInit(): Observable<String> {
    return this.http.post<String>(url+"init", "");
  }

  postEngineMove() {
    return this.http.post(url + "engine_move", "");
  }

  postManualMove(move: Number) {
    return this.http.post(url + "manual_move", "" + move);
  }

  getState(): Observable<Array<Number>> {
    return this.http.get<Array<Number>>(url + "state")
  }
}
