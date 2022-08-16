import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Subscription } from 'rxjs';
import { UserService } from 'src/app/services/user.service';
import { validateForm } from 'src/app/util/utilities';

@Component({
  selector: 'app-user-edit',
  templateUrl: './user-edit.component.html',
  styleUrls: ['./user-edit.component.scss'],
})
export class UserEditComponent implements OnInit, OnDestroy {
  groupForm: FormGroup;
  controlSubscriptions: Array<Subscription>;
  hasChanges: boolean;

  formNameKey = 'user-edit';
  userNameKey = 'userName';
  emailKey = 'email';

  constructor(private userService: UserService) {
    this.controlSubscriptions = new Array<Subscription>();

    this.groupForm = new FormGroup({
      userName: new FormControl<string>('', Validators.required),
      email: new FormControl<string>('', [Validators.required, Validators.email]),
    });
  }

  ngOnInit(): void {
    this.groupForm.get(this.userNameKey).setValue(this.userService.currentUser.name);
    this.groupForm.get(this.emailKey).setValue(this.userService.currentUser.email);

    this.controlSubscriptions.push(
      this.groupForm.valueChanges.subscribe(() => {
        if (this.groupForm.dirty) {
          this.valueChanged();
        }
      })
    );
  }

  ngOnDestroy(): void {
    if (this.controlSubscriptions && this.controlSubscriptions.length > 0) {
      for (const sub of this.controlSubscriptions) {
        sub.unsubscribe();
      }
    }

    this.controlSubscriptions = new Array<Subscription>();
  }

  valueChanged() {
    // Process form validation.
    validateForm(this.formNameKey);

    const userName = this.groupForm.get(this.userNameKey).value as string;
    const email = this.groupForm.get(this.emailKey).value as string;

    if (
      userName &&
      userName.toLowerCase() === this.userService.currentUser.name.toLowerCase() &&
      email &&
      email.toLowerCase() === this.userService.currentUser.email.toLowerCase()
    ) {
      this.hasChanges = false;
    } else {
      this.hasChanges = true;
    }
  }

  saveChanges() {
    this.hasChanges = false;
  }
}
