pub mod fixtures;

use conventional_commit_parser::commit::Separator;
use indoc::indoc;
use itertools::Itertools;
use pretty_assertions::assert_eq;
use speculoos::prelude::*;

use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::Template;
use crate::conventional::changelog::tests::fixtures::{
    default_package_context, default_remote_context, monorepo_context, CommitFixture,
    ReleaseFixture,
};
use crate::git::repository::Repository;

macro_rules! assert_doc_eq {
    ($changelog:expr, $doc:literal) => {
        assert_eq!(
            $changelog.split('\n').map(|line| line.trim()).join("\n"),
            indoc!($doc).split('\n').map(|line| line.trim()).join("\n")
        )
    };
}

macro_rules! changelog_test {
    (
            $test_name:ident,
            $release_fixture:expr,
            $template:expr,
            $expected:literal $(,)?
        ) => {
        #[test]
        fn $test_name() -> anyhow::Result<()> {
            let release = $release_fixture.build();
            let changelog = $template.render(release)?;
            assert_doc_eq!(changelog, $expected);
            Ok(())
        }
    };
    (
            $test_name:ident,
            $release_fixture:expr,
            $template:expr,
            $expected:literal,
            $context:expr $(,)?
        ) => {
        #[test]
        fn $test_name() -> anyhow::Result<()> {
            let release = $release_fixture.build();
            let changelog = $template.with_context($context).render(release)?;
            assert_doc_eq!(changelog, $expected);
            Ok(())
        }
    };
}
#[test]
fn should_get_a_release() -> anyhow::Result<()> {
    let repo = Repository::open(".")?;
    let iter = repo.revwalk("..")?;
    let release = Release::try_from(iter);
    assert_that!(release)
        .is_ok()
        .matches(|r| !r.commits.is_empty());
    Ok(())
}

changelog_test!(
    should_render_default_template,
    ReleaseFixture::default(),
    Template::from_arg("default", None, false)?,
    "## 1.0.0 - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
);

changelog_test!(
        should_render_full_hash_template,
        ReleaseFixture::default(),
        Template::from_arg("full_hash", default_remote_context(), false)?,
        "#### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

changelog_test!(
        should_render_github_template,
        ReleaseFixture::default(),
        Template::from_arg("remote", default_remote_context(), false)?,
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

changelog_test!(
    should_render_template_monorepo,
    ReleaseFixture::default(),
    Template::from_arg("monorepo_default", default_remote_context(), false)?
        .with_context(monorepo_context()),
    "## 1.0.0 - 2015-09-05
        ### Package updates
        - one bumped to 0.1.0
        - two bumped to 0.2.0
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
);

changelog_test!(
    should_render_template_monorepo_for_manual_bump,
    ReleaseFixture::default(),
    Template::from_arg("monorepo_default", None, false)?.with_context(default_package_context()),
    "## 1.0.0 - 2015-09-05
        ### Packages
        - one locked to 0.1.0
        - two locked to 0.2.0
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
);

changelog_test!(
        should_render_full_hash_template_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_full_hash", default_remote_context(), false)?
            .with_context(monorepo_context()),
        "### Package updates
        - one bumped to 0.1.0
        - two bumped to 0.2.0
        ### Global changes
        #### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

changelog_test!(
        should_render_full_hash_template_manual_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_full_hash", default_remote_context(), false)?
            .with_context(default_package_context()),
        "### Packages
        - one locked to 0.1.0
        - two locked to 0.2.0
        ### Global changes
        #### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

changelog_test!(
        should_render_remote_template_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_remote", default_remote_context(), false)?
            .with_context(monorepo_context()),
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        ### Package updates
        - [0.1.0](crates/one) bumped to [0.1.0](https://github.com/cocogitto/cocogitto/compare/0.2.0..0.1.0)
        - [0.2.0](crates/two) bumped to [0.2.0](https://github.com/cocogitto/cocogitto/compare/0.3.0..0.2.0)
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

changelog_test!(
    should_render_template_package,
    ReleaseFixture::default(),
    Template::from_arg("package_default", default_remote_context(), false)?,
    "## 1.0.0 - 2015-09-05
    #### Features
    - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
    - awesome feature - (17f7e23) - Paul Delafosse
    #### Bug Fixes
    - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
    "
);

changelog_test!(
        should_render_full_hash_template_package,
        ReleaseFixture::default(),
        Template::from_arg("full_hash", default_remote_context(), false)?,
        "#### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

changelog_test!(
        should_render_remote_template_package,
        ReleaseFixture::default(),
        Template::from_arg("package_remote", default_remote_context(), false)?,
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

changelog_test!(
        should_render_remote_template_monorepo_for_manual_bump,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_remote", default_remote_context(), false)?
            .with_context(default_package_context()),
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        ### Packages
        - [0.1.0](crates/one) locked to [0.1.0](https://github.com/cocogitto/cocogitto/tree/0.1.0)
        - [0.2.0](crates/two) locked to [0.2.0](https://github.com/cocogitto/cocogitto/tree/0.2.0)
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

changelog_test!(
    should_render_unified_default,
    ReleaseFixture::default(),
    Template::from_arg("default", None, true)?.with_context(monorepo_context()),
    "## 1.0.0 - 2015-09-05
    ### Package updates
    - one bumped to 0.1.0
    - two bumped to 0.2.0
    ### All changes
    #### Features
    - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
    - awesome feature - (17f7e23) - Paul Delafosse
    #### Bug Fixes
    - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
    "
);

changelog_test!(
    should_render_unified_full_hash,
    ReleaseFixture::default(),
    Template::from_arg("full_hash", None, true)?.with_context(monorepo_context()),
    "### Package updates
    - one bumped to 0.1.0
    - two bumped to 0.2.0
    ### All changes
    #### Features
    - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
    - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

    #### Bug Fixes
    - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
    "
);

changelog_test!(
    should_render_unified_remote,
    ReleaseFixture::default(),
    Template::from_arg("remote", default_remote_context(), true)?.with_context(monorepo_context()),
    "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
    ### Package updates
    - [0.1.0](crates/one) bumped to [0.1.0](https://github.com/cocogitto/cocogitto/compare/0.2.0..0.1.0)
    - [0.2.0](crates/two) bumped to [0.2.0](https://github.com/cocogitto/cocogitto/compare/0.3.0..0.2.0)
    ### All changes
    #### Features
    - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
    - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
    #### Bug Fixes
    - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
    "
);
