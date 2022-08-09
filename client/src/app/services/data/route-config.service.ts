import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root',
})
export class RouteConfigService {
  private get baseAPIPath(): string {
    return environment.api_path + 'api';
  }

  public get baseUserPath(): string {
    return this.baseAPIPath + '/users';
  }

  public get me(): string {
    return this.baseUserPath + '/me';
  }

  public get registerUser(): string {
    return this.baseUserPath + '/_register';
  }

  public get loginUser(): string {
    return this.baseUserPath + '/_login';
  }

  public get logOutUser(): string {
    return this.baseUserPath + '/_logout';
  }

  public get changePassword(): string {
    return this.baseUserPath + '/_change_pwd';
  }

  public get baseCategoryPath(): string {
    return this.baseAPIPath + '/category';
  }

  public get baseRecipePath(): string {
    return this.baseAPIPath + '/recipe';
  }

  constructor() {}
}
