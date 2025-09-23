# Pull Request

## Description

Brief description of what this PR does.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring
- [ ] Performance improvement
- [ ] Test improvement

## BitTorrent Protocol Context

- [ ] This relates to the BitTorrent protocol specification
- [ ] This relates to tracker communication
- [ ] This relates to peer-to-peer communication
- [ ] This relates to torrent file parsing
- [ ] This relates to bencode parsing
- [ ] This is a general improvement

## Changes Made

- List the specific changes made
- Be detailed about what was added, removed, or modified

## Testing

- [ ] I have tested this change locally
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] All existing tests pass
- [ ] I have tested with different torrent files
- [ ] I have tested on different operating systems (if applicable)

## Test Commands Used

```bash
# List the commands you used to test
make quick
make test
cargo run --example basic-client -- examples/torrents/debian.iso.torrent
```

## Screenshots/Output

If applicable, add screenshots or command output to help explain your changes.

## Checklist

- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## Related Issues

Closes #(issue number)

## Additional Notes

Any additional information that reviewers should know.
