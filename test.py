import sys
import io
sys.path.append('pyhanko-deps')

from pyhanko.sign import signers
from pyhanko.pdf_utils.incremental_writer import IncrementalPdfFileWriter

cms_signer = signers.SimpleSigner.load(
    'test-certs/server.key', 'test-certs/server.crt',
    ca_chain_files=('test-certs/root/root.crt',),
)

input_document = sys.stdin.buffer.read()
input_document = io.BytesIO(input_document)

w = IncrementalPdfFileWriter(input_document)
output_document = signers.sign_pdf(
    w, signers.PdfSignatureMetadata(field_name='Test_signature'),
    signer=cms_signer,
)

sys.stdout.buffer.write(output_document.getvalue())

