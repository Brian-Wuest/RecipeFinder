import { Injectable } from '@angular/core';
import { RouteConfigService } from './route-config.service';
import { HttpClient } from '@angular/common/http';
import { IGetUserResponse } from 'src/app/models/responses/get-user';
import { Observable } from 'rxjs';
import { ILoginRequest } from 'src/app/models/requests/login-request';
import { IGetMeResponse } from 'src/app/models/responses/get-me';

@Injectable({
  providedIn: 'root',
})
export class UserDataService {
  constructor(private routeConfig: RouteConfigService, private httpClient: HttpClient) {}

  public getMe(): Observable<IGetMeResponse> {
    return this.httpClient.get<IGetMeResponse>(this.routeConfig.me, { withCredentials: true });
  }

  public getAllUsers(): Observable<Array<IGetUserResponse>> {
    return this.httpClient.get<Array<IGetUserResponse>>(this.routeConfig.baseUserPath, { withCredentials: true });
  }

  public login(request: ILoginRequest): Observable<Object> {
    return this.httpClient.post(this.routeConfig.loginUser, request, { withCredentials: true });
  }

  public logout(): Observable<string> {
    return this.httpClient.post(this.routeConfig.logOutUser, null, { withCredentials: true, responseType: "text"});
  }
}
