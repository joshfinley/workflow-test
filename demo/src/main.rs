//! Source: https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs
//! Included to provide testing surface for repository infrastructure. Source
//! license included at the bottom of this file.

//! Observer is a behavioral design pattern that allows one objects to notify
//! other objects about changes in their state.

trait IObserver {
    fn update(&self);
}

trait ISubject<'a, T: IObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify_observers(&self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}
impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    fn new() -> Subject<'a, T> {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn detach(&mut self, observer: &'a T) {
        if let Some(idx) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(idx);
        }
    }
    fn notify_observers(&self) {
        for item in self.observers.iter() {
            item.update();
        }
    }
}

#[derive(PartialEq)]
struct ConcreteObserver {
    id: i32,
}
impl IObserver for ConcreteObserver {
    fn update(&self) {
        println!("Observer id:{} received event!", self.id);
    }
}

// Extracted run_main()
fn run_main() {
    let mut subject = Subject::new();
    let observer_a = ConcreteObserver { id: 1 };
    let observer_b = ConcreteObserver { id: 2 };

    subject.attach(&observer_a);
    subject.attach(&observer_b);
    subject.notify_observers();

    subject.detach(&observer_b);
    subject.notify_observers();
}

fn main() {
    run_main();
}


#[cfg(test)]
mod tests {
    use super::*;

    // A test observer that records if it was updated
    #[derive(PartialEq)]
    struct TestObserver {
        id:      i32,
        updated: std::cell::RefCell<bool>,
    }

    impl TestObserver {
        fn new(id: i32) -> Self {
            Self {
                id,
                updated: std::cell::RefCell::new(false),
            }
        }

        fn was_updated(&self) -> bool {
            *self.updated.borrow()
        }

        fn reset(&self) {
            *self.updated.borrow_mut() = false;
        }
    }

    impl IObserver for TestObserver {
        fn update(&self) {
            *self.updated.borrow_mut() = true;
        }
    }

    #[test]
    fn test_attach_and_notify() {
        let mut subject = Subject::new();
        let observer = TestObserver::new(1);

        subject.attach(&observer);
        subject.notify_observers();

        assert!(observer.was_updated(), "Observer should have been updated");
    }

    #[test]
    fn test_detach() {
        let mut subject = Subject::new();
        let observer = TestObserver::new(2);

        subject.attach(&observer);
        subject.detach(&observer);
        observer.reset();

        subject.notify_observers();

        assert!(
            !observer.was_updated(),
            "Detached observer should NOT have been updated"
        );
    }

    #[test]
    fn test_multiple_observers() {
        let mut subject = Subject::new();
        let observer1 = TestObserver::new(1);
        let observer2 = TestObserver::new(2);

        subject.attach(&observer1);
        subject.attach(&observer2);

        subject.notify_observers();

        assert!(
            observer1.was_updated(),
            "Observer 1 should have been updated"
        );
        assert!(
            observer2.was_updated(),
            "Observer 2 should have been updated"
        );
    }

    #[test]
    fn test_detach_one_of_multiple() {
        let mut subject = Subject::new();
        let observer1 = TestObserver::new(1);
        let observer2 = TestObserver::new(2);

        subject.attach(&observer1);
        subject.attach(&observer2);
        subject.detach(&observer1);
        observer1.reset();
        observer2.reset();

        subject.notify_observers();

        assert!(
            !observer1.was_updated(),
            "Detached observer 1 should NOT have been updated"
        );
        assert!(
            observer2.was_updated(),
            "Observer 2 should have been updated"
        );
    }

    #[test]
    fn test_run_main() {
        // Just make sure run_main() doesn't panic
        run_main();
    }
}


// Mozilla Public License Version 2.0
// ================================
//
//  Definitions
// ------------
//
// 1. "Contributor"
//   means each individual or legal entity that creates, contributes to
//   the creation of, or owns Covered Software.
//
// 2. "Contributor Version"
//   means the combination of the Contributions of others (if any) used
//   by a Contributor and that particular Contributor's Contribution.
//
// 3. "Contribution"
//   means Covered Software of a particular Contributor.
//
// 4. "Covered Software"
//   means Source Code Form to which the initial Contributor has attached
//   the notice in Exhibit A, the Executable Form of such Source Code
//   Form, and Modifications of such Source Code Form, in each case
//   including portions thereof.
//
// 5. "Incompatible With Secondary Licenses"
//   means
//
//   (a) that the initial Contributor has attached the notice described
//       in Exhibit B to the Covered Software; or
//
//   (b) that the Covered Software was made available under the terms of
//       version 1.1 or earlier of the License, but not also under the
//       terms of a Secondary License.
//
// 6. "Executable Form"
//   means any form of the work other than Source Code Form.
//
// 7. "Larger Work"
//   means a work that combines Covered Software with other material, in
//   a separate file or files, that is not Covered Software.
//
// 8. "License"
//   means this document.
//
// 9. "Licensable"
//   means having the right to grant, to the maximum extent possible,
//   whether at the time of the initial grant or subsequently, any and
//   all of the rights conveyed by this License.
//
// 10. "Modifications"
//   means any of the following:
//
//   (a) any file in Source Code Form that results from an addition to,
//       deletion from, or modification of the contents of Covered
//       Software; or
//
//   (b) any new file in Source Code Form that contains any Covered
//       Software.
//
// 11. "Patent Claims" of a Contributor
//   means any patent claim(s), including without limitation, method,
//   process, and apparatus claims, in any patent Licensable by such
//   Contributor that would be infringed, but for the grant of the
//   License, by the making, using, selling, offering for sale, having
//   made, import, or transfer of either its Contributions or its
//   Contributor Version.
//
// 12. "Secondary License"
//   means either the GNU General Public License, Version 2.0, the GNU
//   Lesser General Public License, Version 2.1, the GNU Affero General
//   Public License, Version 3.0, or any later versions of those
//   licenses.
//
// 13. "Source Code Form"
//   means the form of the work preferred for making modifications.
//
// 14. "You" (or "Your")
//   means an individual or a legal entity exercising rights under this
//   License. For legal entities, "You" includes any entity that
//   controls, is controlled by, or is under common control with You. For
//   purposes of this definition, "control" means (a) the power, direct
//   or indirect, to cause the direction or management of such entity,
//   whether by contract or otherwise, or (b) ownership of more than
//   fifty percent (50%) of the outstanding shares or beneficial
//   ownership of such entity.
//
//  License Grants and Conditions
// ------------------------------
//
// 1. Grants
//
// ch Contributor hereby grants You a world-wide, royalty-free,
// n-exclusive license:
//
// ) under intellectual property rights (other than patent or trademark)
//   Licensable by such Contributor to use, reproduce, make available,
//   modify, display, perform, distribute, and otherwise exploit its
//   Contributions, either on an unmodified basis, with Modifications, or
//   as part of a Larger Work; and
//
// ) under Patent Claims of such Contributor to make, use, sell, offer
//   for sale, have made, import, and otherwise transfer either its
//   Contributions or its Contributor Version.
//
// 2. Effective Date
//
// e licenses granted in Section 2.1 with respect to any Contribution
// come effective for each Contribution on the date the Contributor first
// stributes such Contribution.
//
// 3. Limitations on Grant Scope
//
// e licenses granted in this Section 2 are the only rights granted under
// is License. No additional rights or licenses will be implied from the
// stribution or licensing of Covered Software under this License.
// twithstanding Section 2.1(b) above, no patent license is granted by a
// ntributor:
//
// ) for any code that a Contributor has removed from Covered Software;
//   or
//
// ) for infringements caused by: (i) Your and any other third party's
//   modifications of Covered Software, or (ii) the combination of its
//   Contributions with other software (except as part of its Contributor
//   Version); or
//
// ) under Patent Claims infringed by Covered Software in the absence of
//   its Contributions.
//
// is License does not grant any rights in the trademarks, service marks,
//  logos of any Contributor (except as may be necessary to comply with
// e notice requirements in Section 3.4).
//
// 4. Subsequent Licenses
//
//  Contributor makes additional grants as a result of Your choice to
// stribute the Covered Software under a subsequent version of this
// cense (see Section 10.2) or under the terms of a Secondary License (if
// rmitted under the terms of Section 3.3).
//
// 5. Representation
//
// ch Contributor represents that the Contributor believes its
// ntributions are its original creation(s) or it has sufficient rights
//  grant the rights to its Contributions conveyed by this License.
//
// 6. Fair Use
//
// is License is not intended to limit any rights You have under
// plicable copyright doctrines of fair use, fair dealing, or other
// uivalents.
//
// 7. Conditions
//
// ctions 3.1, 3.2, 3.3, and 3.4 are conditions of the licenses granted
//  Section 2.1.
//
//  Responsibilities
// -----------------
//
// 1. Distribution of Source Form
//
// l distribution of Covered Software in Source Code Form, including any
// difications that You create or to which You contribute, must be under
// e terms of this License. You must inform recipients that the Source
// de Form of the Covered Software is governed by the terms of this
// cense, and how they can obtain a copy of this License. You may not
// tempt to alter or restrict the recipients' rights in the Source Code
// rm.
//
// 2. Distribution of Executable Form
//
//  You distribute Covered Software in Executable Form then:
//
// ) such Covered Software must also be made available in Source Code
//   Form, as described in Section 3.1, and You must inform recipients of
//   the Executable Form how they can obtain a copy of such Source Code
//   Form by reasonable means in a timely manner, at a charge no more
//   than the cost of distribution to the recipient; and
//
// ) You may distribute such Executable Form under the terms of this
//   License, or sublicense it under different terms, provided that the
//   license for the Executable Form does not attempt to limit or alter
//   the recipients' rights in the Source Code Form under this License.
//
// 3. Distribution of a Larger Work
//
// u may create and distribute a Larger Work under terms of Your choice,
// ovided that You also comply with the requirements of this License for
// e Covered Software. If the Larger Work is a combination of Covered
// ftware with a work governed by one or more Secondary Licenses, and the
// vered Software is not Incompatible With Secondary Licenses, this
// cense permits You to additionally distribute such Covered Software
// der the terms of such Secondary License(s), so that the recipient of
// e Larger Work may, at their option, further distribute the Covered
// ftware under the terms of either this License or such Secondary
// cense(s).
//
// 4. Notices
//
// u may not remove or alter the substance of any license notices
// ncluding copyright notices, patent notices, disclaimers of warranty,
//  limitations of liability) contained within the Source Code Form of
// e Covered Software, except that You may alter any license notices to
// e extent required to remedy known factual inaccuracies.
//
// 5. Application of Additional Terms
//
// u may choose to offer, and to charge a fee for, warranty, support,
// demnity or liability obligations to one or more recipients of Covered
// ftware. However, You may do so only on Your own behalf, and not on
// half of any Contributor. You must make it absolutely clear that any
// ch warranty, support, indemnity, or liability obligation is offered by
// u alone, and You hereby agree to indemnify every Contributor for any
// ability incurred by such Contributor as a result of warranty, support,
// demnity or liability terms You offer. You may include additional
// sclaimers of warranty and limitations of liability specific to any
// risdiction.
//
//  Inability to Comply Due to Statute or Regulation
// -------------------------------------------------
//
//  it is impossible for You to comply with any of the terms of this
// cense with respect to some or all of the Covered Software due to
// atute, judicial order, or regulation then You must: (a) comply with
// e terms of this License to the maximum extent possible; and (b)
// scribe the limitations and the code they affect. Such description must
//  placed in a text file included with all distributions of the Covered
// ftware under this License. Except to the extent prohibited by statute
//  regulation, such description must be sufficiently detailed for a
// cipient of ordinary skill to be able to understand it.
//
//  Termination
// ------------
//
// 1. The rights granted under this License will terminate automatically
//  You fail to comply with any of its terms. However, if You become
// mpliant, then the rights granted under this License from a particular
// ntributor are reinstated (a) provisionally, unless and until such
// ntributor explicitly and finally terminates Your grants, and (b) on an
// going basis, if such Contributor fails to notify You of the
// n-compliance by some reasonable means prior to 60 days after You have
// me back into compliance. Moreover, Your grants from a particular
// ntributor are reinstated on an ongoing basis if such Contributor
// tifies You of the non-compliance by some reasonable means, this is the
// rst time You have received notice of non-compliance with this License
// om such Contributor, and You become compliant prior to 30 days after
// ur receipt of the notice.
//
// 2. If You initiate litigation against any entity by asserting a patent
// fringement claim (excluding declaratory judgment actions,
// unter-claims, and cross-claims) alleging that a Contributor Version
// rectly or indirectly infringes any patent, then the rights granted to
// u by any and all Contributors for the Covered Software under Section
// 1 of this License shall terminate.
//
// 3. In the event of termination under Sections 5.1 or 5.2 above, all
// d user license agreements (excluding distributors and resellers) which
// ve been validly granted by You or Your distributors under this License
// ior to termination shall survive termination.
//
// **********************************************************************
//                                                                      *
//  6. Disclaimer of Warranty                                           *
//  -------------------------                                           *
//                                                                      *
//  Covered Software is provided under this License on an "as is"       *
//  basis, without warranty of any kind, either expressed, implied, or  *
//  statutory, including, without limitation, warranties that the       *
//  Covered Software is free of defects, merchantable, fit for a        *
//  particular purpose or non-infringing. The entire risk as to the     *
//  quality and performance of the Covered Software is with You.        *
//  Should any Covered Software prove defective in any respect, You     *
//  (not any Contributor) assume the cost of any necessary servicing,   *
//  repair, or correction. This disclaimer of warranty constitutes an   *
//  essential part of this License. No use of any Covered Software is   *
//  authorized under this License except under this disclaimer.         *
//                                                                      *
// **********************************************************************
//
// **********************************************************************
//                                                                      *
//  7. Limitation of Liability                                          *
//  --------------------------                                          *
//                                                                      *
//  Under no circumstances and under no legal theory, whether tort      *
//  (including negligence), contract, or otherwise, shall any           *
//  Contributor, or anyone who distributes Covered Software as          *
//  permitted above, be liable to You for any direct, indirect,         *
//  special, incidental, or consequential damages of any character      *
//  including, without limitation, damages for lost profits, loss of    *
//  goodwill, work stoppage, computer failure or malfunction, or any    *
//  and all other commercial damages or losses, even if such party      *
//  shall have been informed of the possibility of such damages. This   *
//  limitation of liability shall not apply to liability for death or   *
//  personal injury resulting from such party's negligence to the       *
//  extent applicable law prohibits such limitation. Some               *
//  jurisdictions do not allow the exclusion or limitation of           *
//  incidental or consequential damages, so this exclusion and          *
//  limitation may not apply to You.                                    *
//                                                                      *
// **********************************************************************
//
//  Litigation
// -----------
//
// y litigation relating to this License may be brought only in the
// urts of a jurisdiction where the defendant maintains its principal
// ace of business and such litigation shall be governed by laws of that
// risdiction, without reference to its conflict-of-law provisions.
// thing in this Section shall prevent a party's ability to bring
// oss-claims or counter-claims.
//
//  Miscellaneous
// --------------
//
// is License represents the complete agreement concerning the subject
// tter hereof. If any provision of this License is held to be
// enforceable, such provision shall be reformed only to the extent
// cessary to make it enforceable. Any law or regulation which provides
// at the language of a contract shall be construed against the drafter
// all not be used to construe this License against a Contributor.
//
// . Versions of the License
// -------------------------
//
// .1. New Versions
//
// zilla Foundation is the license steward. Except as provided in Section
// .3, no one other than the license steward has the right to modify or
// blish new versions of this License. Each version will be given a
// stinguishing version number.
//
// .2. Effect of New Versions
//
// u may distribute the Covered Software under the terms of the version
//  the License under which You originally received the Covered Software,
//  under the terms of any subsequent version published by the license
// eward.
//
// .3. Modified Versions
//
//  you create software not governed by this License, and you want to
// eate a new license for such software, you may create and use a
// dified version of this License if you rename the license and remove
// y references to the name of the license steward (except to note that
// ch modified license differs from this License).
//
// .4. Distributing Source Code Form that is Incompatible With Secondary
// censes
//
//  You choose to distribute Source Code Form that is Incompatible With
// condary Licenses under the terms of this version of the License, the
// tice described in Exhibit B of this License must be attached.
//
// hibit A - Source Code Form License Notice
// -----------------------------------------
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//  it is not possible or desirable to put the notice in a particular
// le, then You may include the notice in a location (such as a LICENSE
// le in a relevant directory) where a recipient would be likely to look
// r such a notice.
//
// u may add additional accurate notices of copyright ownership.
//
// hibit B - "Incompatible With Secondary Licenses" Notice
// -------------------------------------------------------
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
