import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

not_run: for (def oaid : ('524'..'527')) {
}

WebUI.openBrowser('https://hr.aetest.bwae.org/kpi-salary-ui/#/')

WebUI.navigateToUrl('https://hr.aetest.bwae.org/kpi-salary-ui/#/')

WebUI.setText(findTestObject('Object Repository/login/Page_/input__el-input__inner'), oaid)

WebUI.click(findTestObject('Object Repository/login/Page_/div_'))

not_run: WebUI.verifyCheckpoint(findCheckpoint('Checkpoints/new'), oaid)

WebUI.verifyElementPresent(findTestObject('login/Page_/caidan'), 6)

WebUI.closeBrowser()

