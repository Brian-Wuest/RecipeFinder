import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { map, Observable, of } from 'rxjs';
import { AuthorizationRoleClassification } from 'src/app/models/enums/auth-role-classification.enum';
import { IGetAuthorization } from 'src/app/models/responses/get-authorization';
import { RouteConfigService } from './route-config.service';

@Injectable({
  providedIn: 'root',
})
export class AuthorizationDataService {
  constructor(private routeConfig: RouteConfigService, private httpClient: HttpClient) {}

  public getAuthorizationRole(): Observable<IGetAuthorization> {
    return this.httpClient.get<IGetAuthorization>(this.routeConfig.baseAuthorizationPath, { withCredentials: true });
  }
}
