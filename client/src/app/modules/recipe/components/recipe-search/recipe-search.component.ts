import { Component, OnInit } from '@angular/core';
import { UserDataService } from 'src/app/services/data/user-data.service';

@Component({
  selector: 'app-recipe-search',
  templateUrl: './recipe-search.component.html',
  styleUrls: ['./recipe-search.component.scss']
})
export class RecipeSearchComponent implements OnInit {

  constructor(private userDataService: UserDataService) { }

  ngOnInit(): void {
  }

  getUsers() {
    this.userDataService.getAllUsers().subscribe(result => {
      console.log("User Result", result);
    });
  }

}
